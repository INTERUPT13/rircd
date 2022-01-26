use color_eyre::Result;
use crate::endpoint::EndpointBackend;
use async_trait::async_trait;
use log::{info,debug,trace};

use ellidri_tokens::Message as IrcMessage;

use futures::StreamExt;

use std::pin::Pin;
use std::net::{SocketAddr};

use tokio::net::{TcpStream, TcpListener};
use futures::stream::FuturesUnordered;

use tokio::io::AsyncReadExt;

struct IrcClientConnection {
}

impl IrcClientConnection {
    pub async fn handle(mut connection: TcpStream, addr: SocketAddr) {
        trace!("connection handler for {}", addr);
        // TODO make this user configureable. Though 512 should be the default msg size and 4096
        // should be the max space tags can use in ircv3 by spec
        let mut buf = [0;512 + 4096];
        loop {
            let bytes_read = match connection.read(&mut buf).await {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    debug!("couldn't read data from {}'s socket as of: {}", addr, e);
                    continue;
                },
            };
            if bytes_read == 0 {
                break;
            }
            let msg_str = match std::str::from_utf8(&buf[0..bytes_read]) {
                Ok(msg) => msg,
                Err(e) => {
                    debug!("error converting {}'s message to the \"str\" type: {}", addr, e);
                    continue;
                },
            };

            let msg_parsed = match IrcMessage::parse(msg_str) {
                None => {
                    debug!("error parsing {}'s message to a valid IRC command", addr);
                    continue;
                },
                Some(msg_parsed) => {
                    debug!("{} issued command {:?}", addr, msg_parsed);
                    msg_parsed
                },
            };




        }

        info!("closing connection to {}", addr);
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
                    tokio::spawn( IrcClientConnection::handle(conn.0, conn.1) );
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


