#![feature(impl_trait_in_assoc_type)]

use anyhow::anyhow;
use futures::future::select_all;
use std::collections::HashMap;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use volo::FastStr;
use volo_gen::mini_redis::DelRequest;
pub mod arg;
pub mod parse;
use volo_gen::mini_redis::SubscribeResponse;
pub struct S {
    pub master: bool,
    pub port: u16,
    pub map: Arc<Mutex<HashMap<String, String>>>,
    pub channels: Mutex<HashMap<String, broadcast::Sender<String>>>,
    pub file: Option<Arc<tokio::sync::Mutex<std::fs::File>>>,
    pub slaves: Mutex<Vec<volo_gen::mini_redis::RedisServiceClient>>,
    pub buf: Arc<tokio::sync::Mutex<Vec<String>>>,
    pub handles: Arc<tokio::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

pub struct Proxy {
    pub clients: tokio::sync::Mutex<Vec<Vec<volo_gen::mini_redis::RedisServiceClient>>>,
    pub count: Vec<tokio::sync::Mutex<usize>>,
    pub servers: Vec<Vec<String>>,
    pub id: tokio::sync::Mutex<i32>,
    pub transactions: tokio::sync::Mutex<HashMap<i32, Vec<(Option<volo_gen::mini_redis::SetRequest>,Option<volo_gen::mini_redis::DelRequest>)>>>,
}
pub struct P {}

// impl Drop for S {
//     fn drop(&mut self) {
//         let file_clone = Arc::clone(&self.file.clone().unwrap());
//         let buf_clone = Arc::clone(&self.buf);
//         let handle = tokio::spawn(async move {
//                 let mut file = file_clone.lock().await;
//                 let mut buf = buf_clone.lock().await;
//                 file.write_all((buf.join("\n") + "\n").as_bytes())
//                     .unwrap();
//                 buf.clear();
//         });
//         self.handles.lock().await.push(handle);
//         tokio::join!(async {
//             for handle in self.handles.lock().await.iter_mut() {
//                 handle.await.unwrap();
//             }
//         });
//     }
// }

#[volo::async_trait]
impl volo_gen::mini_redis::RedisService for Proxy {
    async fn set(
        &self,
        _request: volo_gen::mini_redis::SetRequest,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        if _request.transaction_id.is_some() {
            let mut transactions = self.transactions.lock().await;
            let mut id = self.id.lock().await;
            if !transactions.contains_key(&_request.transaction_id.clone().unwrap()) {
                transactions.insert(*id, vec![]);
                *id += 1;
            }
            transactions
                .get_mut(&_request.transaction_id.clone().unwrap())
                .unwrap()
                .push((Some(_request.clone()),None));
            return Ok("Ok".into());
        }
        let clients = self.clients.lock().await;
        let idx1 = _request.key.as_str().as_bytes()[0] as usize % clients.len();
        println!("Request is forwarded to port {}", self.servers[idx1][0]);
        let client =
            clients[_request.key.as_str().as_bytes()[0] as usize % clients.len()][0].clone();
        match client.set(_request).await {
            Err(e) => Err(e.into()),
            Ok(res) => Ok(res),
        }
    }
    async fn get(
        &self,
        _key: FastStr,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        let clients = self.clients.lock().await;
        let idx1 = _key.as_str().as_bytes()[0] as usize % clients.len();
        let idx2 = *self.count[idx1].lock().await % (clients[idx1].len() - 1) + 1;
        *self.count[idx1].lock().await += 1;
        println!("Request is forwarded to port {}", self.servers[idx1][idx2]);
        let client = clients[idx1][idx2].clone();
        let resp = client.get(_key).await;
        match resp {
            Err(e) => Err(e.into()),
            Ok(res) => Ok(res),
        }
    }
    async fn del(
        &self,
        _request: volo_gen::mini_redis::DelRequest,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        let clients = self.clients.lock().await;
        let idx1 = _request.key.as_str().as_bytes()[0] as usize % clients.len();
        let client = clients[idx1][0].clone();
        println!("Request is forwarded to port {}", self.servers[idx1][0]);
        match client.del(_request).await {
            Err(e) => Err(e.into()),
            Ok(res) => Ok(res),
        }
    }
    async fn ping(
        &self,
        _message: FastStr,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        let clients = self.clients.lock().await;
        let client = clients[0][1].clone();
        match client.ping(_message).await {
            Err(e) => Err(e.into()),
            Ok(res) => Ok(res),
        }
    }
    async fn subscribe(
        &self,
        _request: volo_gen::mini_redis::SubsrcibeRequest,
    ) -> ::core::result::Result<volo_gen::mini_redis::SubscribeResponse, ::volo_thrift::AnyhowError>
    {
        Ok(SubscribeResponse {
            message: "Ok".into(),
            trap: true,
        })
    }
    async fn publish(
        &self,
        _request: volo_gen::mini_redis::PublishRequest,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        Ok("Ok".into())
    }
    async fn register(
        &self,
        _port: FastStr,
    ) -> ::core::result::Result<HashMap<FastStr, FastStr>, ::volo_thrift::AnyhowError> {
        Err(anyhow!("Cant register on slave").into())
    }
    async fn multi(
        &self,
    ) -> ::core::result::Result<i32, ::volo_thrift::AnyhowError> {
        let mut transactions = self.transactions.lock().await;
        let mut id = self.id.lock().await;
        transactions.insert(*id, vec![]);
        *id += 1;
        Ok(*id )
    }
    async fn exec(
        &self,
        _transaction_id: i32,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        Err(anyhow!("Cant exec on slave").into())
    }
    async fn watch(
        &self,
        _key: FastStr,
        _transaction_id: i32,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        Err(anyhow!("Cant watch on slave").into())
    }
}

#[volo::async_trait]
impl volo_gen::mini_redis::RedisService for S {
    async fn set(
        &self,
        _request: volo_gen::mini_redis::SetRequest,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        if !self.master && !_request.sync {
            return Err(anyhow!(" Cant write on slave").into());
        }
        if _request.expire_time.is_some() {
            let map_clone = self.map.clone();
            let key = _request.key.clone().into_string();
            let expire_time = _request.expire_time.clone().unwrap();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(expire_time as u64)).await;
                let mut map = map_clone.lock().unwrap();
                map.remove(&key);
            });
        }
        self.map.lock().unwrap().insert(
            _request.key.clone().into_string(),
            _request.value.clone().into_string(),
        );
        if self.master {
            for slave in self.slaves.lock().unwrap().iter_mut() {
                let slave_clone = slave.clone();
                let _request_new = volo_gen::mini_redis::SetRequest {
                    sync: true,
                    .._request.clone()
                };
                tokio::spawn(async move {
                    let _ = slave_clone.set(_request_new).await;
                });
            }
            let mut buf = self.buf.lock().await;
            if _request.expire_time.is_none() {
                buf.push(format!(
                    "set {} {}",
                    _request.key.into_string(),
                    _request.value.into_string()
                ))
            };
            if buf.len() >= 5 {
                let buf_clone = Arc::clone(&self.buf);
                let file_clone = Arc::clone(&self.file.clone().unwrap());
                let handle = tokio::spawn(async move {
                    let mut buf = buf_clone.lock().await;
                    let mut file = file_clone.lock().await;
                    file.write_all((buf.join("\n") + "\n").as_bytes()).unwrap();
                    buf.clear();
                });
                self.handles.lock().await.push(handle);
                // self.file
                //     .as_ref()
                //     .await
                //     .lock()
                //     .unwrap()
                //     .write_all((buf.join("\n") + "\n").as_bytes())
                //     .unwrap();
                // buf.clear();
            }
        }
        Ok("Ok".into())
    }
    async fn get(
        &self,
        _key: FastStr,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        if self.master {
            return Err(anyhow!(" Cant read on master"));
        }
        match self.map.lock().unwrap().get(&_key.into_string()) {
            Some(v) => Ok(FastStr::from(v.clone())),
            None => Ok("nil".into()),
        }
    }
    async fn del(
        &self,
        _request: volo_gen::mini_redis::DelRequest,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        if !self.master && !_request.sync {
            return Err(anyhow!(" Cant write on slave").into());
        }
        if self.master {
            for slave in self.slaves.lock().unwrap().iter_mut() {
                let slave_clone = slave.clone();
                let _request_new = DelRequest {
                    key: _request.key.clone(),
                    sync: true,
                };
                tokio::spawn(async move {
                    let _ = slave_clone.del(_request_new).await;
                });
            }
        }
        if self.master {
            let mut buf = self.buf.lock().await;
            buf.push(format!("del {}", _request.key.clone().into_string(),));
            if buf.len() >= 5 {
                let buf_clone = Arc::clone(&self.buf);
                let file_clone = Arc::clone(&self.file.clone().unwrap());
                let handle = tokio::spawn(async move {
                    let mut buf = buf_clone.lock().await;
                    let mut file = file_clone.lock().await;
                    file.write_all((buf.join("\n") + "\n").as_bytes()).unwrap();
                    buf.clear();
                });
                self.handles.lock().await.push(handle);
            }
        }
        match self.map.lock().unwrap().remove(&_request.key.into_string()) {
            Some(_) => Ok("Ok".into()),
            None => Ok("nil".into()),
        }
    }
    async fn ping(
        &self,
        _message: FastStr,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        match self.master {
            false => Ok(_message),
            true => Err(anyhow!(" Cant ping on slave")),
        }
    }
    async fn subscribe(
        &self,
        _request: volo_gen::mini_redis::SubsrcibeRequest,
    ) -> ::core::result::Result<volo_gen::mini_redis::SubscribeResponse, ::volo_thrift::AnyhowError>
    {
        match _request.block {
            true => {
                let mut vec = self
                    .channels
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|(k, _v)| _request.channels.contains(&FastStr::from((*k).clone())))
                    .map(|(k, v)| (v.subscribe(), k.clone()))
                    .collect::<Vec<_>>();
                let (res, index, _) =
                    select_all(vec.iter_mut().map(|(rx, _name)| Box::pin(rx.recv()))).await;
                match res {
                    Ok(info) => Ok(SubscribeResponse {
                        message: (String::from("from ") + &vec[index].1 + ": " + &info).into(),

                        trap: true,
                    }),
                    Err(_) => Ok(SubscribeResponse {
                        message: "Error".into(),
                        trap: true,
                    }),
                }
            }
            false => {
                for channel in _request.channels {
                    if !self
                        .channels
                        .lock()
                        .unwrap()
                        .contains_key(&channel.clone().into_string())
                    {
                        let (tx, _) = broadcast::channel(10);
                        self.channels
                            .lock()
                            .unwrap()
                            .insert(channel.clone().into_string(), tx);
                    }
                }
                Ok(SubscribeResponse {
                    message: "Ok".into(),
                    trap: true,
                })
            }
        }
    }
    async fn publish(
        &self,
        _request: volo_gen::mini_redis::PublishRequest,
    ) -> ::core::result::Result<FastStr, ::volo_thrift::AnyhowError> {
        let channel = _request.channel.clone().into_string();
        let _ = self
            .channels
            .lock()
            .unwrap()
            .get(&channel)
            .unwrap()
            .send(_request.message.into_string())
            .unwrap();
        Ok("Ok".into())
    }
    async fn register(
        &self,
        _port: FastStr,
    ) -> ::core::result::Result<HashMap<FastStr, FastStr>, ::volo_thrift::AnyhowError> {
        self.slaves.lock().unwrap().push({
            let addr: SocketAddr = ("127.0.0.1:".to_owned() + _port.as_str()).parse().unwrap();
            volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
                // .layer_outer(LogLayer)
                .layer_outer(FilterLayer)
                .address(addr)
                .build()
        });
        Ok(self
            .map
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| (FastStr::from(k.clone()), FastStr::from(v.clone())))
            .collect())
    }
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{:?}", req);
        if info.contains("Genshin") {
            Err(anyhow::anyhow!("Genshin is forbidden. OP show show way!").into())
        } else {
            self.0.call(cx, req).await
        }
    }
}

pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
