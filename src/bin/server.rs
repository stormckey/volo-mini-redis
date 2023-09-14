#![feature(impl_trait_in_assoc_type)]

use mini_redis::arg::{ServerArgs, ServerType};
use mini_redis::parse::parse;
use mini_redis::FilterLayer;
use mini_redis::{Proxy, S};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use structopt::StructOpt;

use std::io::Write;
use tokio::sync::broadcast;
#[volo::main]
async fn main() {
    let args = ServerArgs::from_args();
    let myport = args.port;
    let addr: SocketAddr = ("[::]:".to_owned() + &myport).parse().unwrap();
    let addr = volo::net::Address::from(addr);
    let (_proxy, servers) = parse();
    let pre_buf = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let pre_handle = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    match args.server_type {
        ServerType::Proxy {} => {
            volo_gen::mini_redis::RedisServiceServer::new({
                Proxy {
                    clients: {
                        let clients = tokio::sync::Mutex::new(Vec::new());
                        for v in &servers {
                            let mut vec_client = Vec::new();
                            let mut first = true;
                            for port in v {
                                // vec_client.push(port)
                                vec_client.push({
                                    let addr: SocketAddr =
                                        ("127.0.0.1:".to_owned() + &port).parse().unwrap();
                                    volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
                                        // .layer_outer(LogLayer)
                                        .layer_outer(FilterLayer)
                                        .address(addr)
                                        .build()
                                });
                                println!(
                                    "Connect to {} Server at port {}.",
                                    if first { "Master" } else { "Slave " },
                                    port
                                );
                                first = false;
                            }
                            clients.lock().await.push(vec_client);
                        }
                        clients
                    },
                    count: {
                        let mut vec = Vec::new();
                        for _ in 0..servers.len() {
                            vec.push(tokio::sync::Mutex::new(0));
                        }
                        vec
                    },
                    servers,
                    id: tokio::sync::Mutex::new(0),
                    transactions: tokio::sync::Mutex::new(HashMap::new()),
                }
            }) // .layer_front(LogLayer)
            .layer_front(FilterLayer)
            .run(addr)
            .await
            .unwrap();
            ()
        }
        ms => {
            let ser = volo_gen::mini_redis::RedisServiceServer::new({
                let mut s = S {
                    master: match ms {
                        ServerType::Master {} => true,
                        _ => false,
                    },
                    port: myport.parse().unwrap(),
                    map: Arc::new(Mutex::new(HashMap::<String, String>::new())),
                    channels: Mutex::new(HashMap::<String, broadcast::Sender<String>>::new()),
                    file: match ms {
                        ServerType::Master {} => Some(Arc::new(tokio::sync::Mutex::new(
                            OpenOptions::new()
                                .append(true)
                                .read(true)
                                .create(true)
                                .open(myport.clone() + &".log")
                                .expect("Cant open file"),
                        ))),
                        _ => None,
                    },
                    slaves: Mutex::new(Vec::new()),
                    // buf: Arc::new(tokio::sync::Mutex::new(Vec::new())),
                    buf: Arc::clone(&pre_buf),
                    // handles: Arc::new(tokio::sync::Mutex::new(Vec::new())),
                    handles: Arc::clone(&pre_handle),
                };
                match (&s.file, &ms) {
                    (Some(ref file), _) => {
                        let file = Arc::clone(&file);
                        let mut file = file.lock().await;

                        for line in BufReader::new(&mut *file).lines() {
                            match line {
                                Ok(info) => {
                                    if info.trim().is_empty() {
                                        continue;
                                    }
                                    let mut args = info.split_whitespace();
                                    let cmd = args.next().unwrap();
                                    let key = args.next().unwrap();
                                    match cmd {
                                        "set" => {
                                            let value = args.next().unwrap();
                                            s.map
                                                .lock()
                                                .unwrap()
                                                .insert(key.to_string(), value.to_string());
                                        }
                                        "del" => {
                                            s.map.lock().unwrap().remove(key);
                                        }
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                }
                            }
                        }
                    }
                    (_, ServerType::Slave { port }) => {
                        let clinet = volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
                            .address(
                                ("127.0.0.1:".to_owned() + &port)
                                    .parse::<SocketAddr>()
                                    .unwrap(),
                            )
                            .build();
                        s.map = Arc::new(Mutex::new(
                            clinet
                                .register(myport.clone().into())
                                .await
                                .unwrap_or_default()
                                .iter()
                                .map(|(key, value)| {
                                    (key.clone().into_string(), value.clone().into_string())
                                })
                                .collect(),
                        ));
                    }
                    _ => {}
                }
                s
            })
            .layer_front(FilterLayer).run(addr).await.unwrap();

            let buf_clone = Arc::clone(&pre_buf);
            let handle = tokio::spawn(async move {
                let mut file = OpenOptions::new()
                    .append(true)
                    .read(true)
                    .create(true)
                    .open(myport.clone() + &".log")
                    .expect("Cant open file");
                let buf = buf_clone.lock().await;
                file.write_all((buf.join("\n") + "\n").as_bytes()).unwrap();
            });
            pre_handle.lock().await.push(handle);
            tokio::join!(async {
                for handle in pre_handle.lock().await.iter_mut() {
                    handle.await.unwrap();
                }
            });
        }
    }
}
