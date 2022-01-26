use color_eyre::Result;
use crate::endpoint::EndpointBackend;
use async_trait::async_trait;
use log::{info,debug,trace};

use futures::StreamExt;

use std::pin::Pin;
use std::net::{SocketAddr, TcpStream};

use tokio::net::TcpListener;
use futures::stream::FuturesUnordered;

struct IrcClientConnection {
}

impl IrcClientConnection {
    pub async fn handle(connection: TcpStream, addr: SocketAddr) {
        trace!("connection handler for {}", addr);
    }
}

pub struct IrcEndpoint {
    client_connections: Vec<IrcClientConnection>,
    bind_addrs: Vec<SocketAddr>,
    sockets: Vec<TcpListener>,
}

impl Default for IrcEndpoint {
    fn default() -> Self {
        Self {
            client_connections: Vec::new(),
            bind_addrs: vec![ "0.0.0.0:6697".parse().unwrap(), "0.0.0.0:7000".parse().unwrap() ],
            sockets: Vec::new(),
        }
    }
}

impl IrcEndpoint {
    async fn run(self) {
        loop {
            // TODO I don't think thats the way to do it
            let mut incomming_connections: FuturesUnordered<_> = self.sockets.iter().map(|sock| sock.accept()).collect();


            tokio::select! {
                conn = incomming_connections.next() => {
                    let conn = match conn {
                        None => {
                            debug!("accepted connection seems to be invalid. Closing connection");
                            continue;
                        },
                        Some(Err(e)) => {
                            debug!("encountered error ");
                            continue;
                        },
                        Some(Ok(c)) => c,
                    };

                    info!("accepted connection from {}", conn.1);
                }
            }
            
        }
    }
}

#[async_trait]
impl EndpointBackend for IrcEndpoint {
    async fn start(mut self: Box<Self>, mut name: String) -> Result<()> {
        info!("starting IRC endpoint {}", name);

        let mut binding_string = String::new();
        
        // TODO this should be faster than just binding to the sockets in
        // an sync fashion one after another (is it really?!)
        let socket_results = {
            let socket_futs: FuturesUnordered<_> = self.bind_addrs.iter().map(|addr| {
                binding_string.push_str(&format!("\n{}",addr));
                Box::pin(TcpListener::bind(addr))
            }).collect();
            socket_futs.collect::<Vec<_>>().await
        };

        // TODO there must be a nicer way to do this
        for socket_result in socket_results {
            self.sockets.push( socket_result? );
        }

        info!("IRC endpoint {} binding to: {}", name, binding_string);

        self.run().await;
        Ok(())
    }

    //async fn connected_users()
}


