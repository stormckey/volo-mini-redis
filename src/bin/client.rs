use mini_redis::arg::{Args, Opt};
use mini_redis::FilterLayer;
use std::net::SocketAddr;
use std::sync::Arc;
use structopt::StructOpt;
use volo::FastStr;
use volo_gen::mini_redis::{DelRequest, PublishRequest, SetRequest, SubsrcibeRequest};

#[volo::main]
async fn main() {
    let args = Args::from_args();
    // tracing_subscriber::fmt::init();
    // let mut args: Vec<String> = std::env::args().collect();
    let client: volo_gen::mini_redis::RedisServiceClient = {
        let addr: SocketAddr = ("127.0.0.1:".to_owned() + &args.port).parse().unwrap();
        volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
            // .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
    match args.cmd {
        Opt::Subscribe { channel, and } => {
            let mut channels = vec![channel];
            if let Some(and) = and {
                channels.extend(and);
            }
            let req = SubsrcibeRequest {
                channels: channels
                    .drain(..)
                    .map(|x| FastStr::from(Arc::new(x)))
                    .collect(),
                block: false,
            };
            let resp = client.subscribe(req.clone()).await;
            match resp {
                Ok(res) => {
                    if res.trap {
                        println!("subscribe {} channels", req.channels.len());
                        loop {
                            let req = SubsrcibeRequest {
                                block: true,
                                ..req.clone()
                            };
                            let resp = client.subscribe(req).await;
                            match resp {
                                Ok(res) => {
                                    println!("{}", res.message);
                                }
                                _ => {
                                    println!("error");
                                }
                            }
                        }
                    }
                }

                Err(e) => {
                    tracing::error!("{:?}", e);
                }
            };
        }
        opt => {
            match match opt {
                Opt::Set { key, value, ex } => {
                    client
                        .set(SetRequest {
                            key: FastStr::from(Arc::new(key)),
                            value: FastStr::from(Arc::new(value)),
                            expire_time: ex,
                            sync: false,
                        })
                        .await
                }
                Opt::Del { key } => {
                    client
                        .del(DelRequest {
                            key: FastStr::from(Arc::new(key)),
                            sync: false,
                        })
                        .await
                }
                Opt::Get { key } => client.get(key.into()).await,
                Opt::Ping { value } => {
                    client
                        .ping(value.unwrap_or(String::from("PONG")).into())
                        .await
                }
                Opt::Publish { channel, value } => {
                    client
                        .publish(PublishRequest {
                            channel: FastStr::from(Arc::new(channel)),
                            message: FastStr::from(Arc::new(value)),
                        })
                        .await
                }
                _ => {
                    unreachable!()
                }
            } {
                Ok(info) => {
                    println!("{}", info);
                }
                Err(e) => match e {
                    volo_thrift::ResponseError::Application(err) => {
                        println!("{}", err.message)
                    }
                    _ => {
                        tracing::error!("{:?}", e);
                    }
                },
            }
        }
    }
}
