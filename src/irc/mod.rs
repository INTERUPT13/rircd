use color_eyre::Result;
use crate::endpoint::EndpointBackend;
use async_trait::async_trait;
use log::{error, info,debug,trace};

use ellidri_tokens::Message as IrcMessage;

use crate::event::{ServerEvent, EndpointEvent};


use crate::message::Message;
use futures::StreamExt;

use color_eyre::eyre::eyre;

use std::pin::Pin;
use std::net::{SocketAddr};

use tokio::sync::{mpsc, oneshot, RwLock};

use tokio::net::{TcpStream, TcpListener};
use futures::stream::FuturesUnordered;

use tokio::io::AsyncReadExt;

use crate::irc::user::IrcUser;

use std::sync::Arc;

mod user;
mod event;
mod connection;

use std::collections::HashMap;
use crate::irc::connection::IrcClientConnection;

// structure that holds IrcEndpoint specific data. Such as irc channels/users/etc. The these
// values are somewhat influenced by the servers abstracted endpoint indepenend channel/user
// records. So for example the title would be changed in the servers abstraced channel
// representation the title here would be changed as well and the users would be notified
pub struct IrcEndpoint {
    client_connections: Vec<IrcClientConnection>,
    bind_addrs: Vec<SocketAddr>,
    sockets: Vec<TcpListener>,

    irc_users: HashMap<SocketAddr,IrcUser>,
}

impl Default for IrcEndpoint {
    fn default() -> Self {
        Self {
            client_connections: Vec::new(),
            bind_addrs: vec![ "0.0.0.0:6697".parse().unwrap(), "0.0.0.0:7000".parse().unwrap() ],
            sockets: Vec::new(),
            irc_users: HashMap::new(),
        }
    }
}

impl IrcEndpoint {
    async fn run(mut self, mut endpoint_event_source: mpsc::Receiver<EndpointEvent>) {
        let (irc_event_sink, mut irc_event_source) = mpsc::channel(99); // TODO configurable size

        loop {
            // TODO I don't think thats the way to do it
            //!                  V
            //!        actually I found out 
            //!  why not just spawn off a acceptor thread for each
            //!  socket instead of this FuturesUnordered
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

                    let (irc_session_event_sink, irc_session_event_source) = mpsc::channel(99); //  TODO configureable
                    self.client_connections.push( IrcClientConnection {
                        //associated_user: Arc::new( RwLock::new( IrcUser {} )),
                        irc_session_event_sink,
                    });
                    tokio::spawn( IrcClientConnection::handle(conn.0, conn.1, irc_event_sink.clone(), irc_session_event_source) );
                }

                irc_event = irc_event_source.recv() => {
                    let irc_event = match irc_event {
                        Some(ev) => ev,
                        None => {
                            error!("received invalid IrcEvent in IrcEndpoint handler");
                            continue;
                        },
                    };
                    trace!("irc endpoint event handle received irc_event {:?}", irc_event);

                    use crate::irc::event::IrcEventType::*;
                    
                    let is_registered = self.irc_users.contains_key(&irc_event.remote_addr);
                    if !is_registered {
                        match irc_event.event {
                            Nick(username) => {
                                self.irc_users.insert( irc_event.remote_addr, IrcUser::from_username(&username));
                            },
                            User(username,realname) => {
                                self.irc_users.insert( irc_event.remote_addr, IrcUser::from_username_and_realname(&username, &realname) );
                            }
                            _ => {
                                debug!("irc client {} attempts to use commands without registering himself via NICK/USER",
                                    irc_event.remote_addr);
                                // TODO tell user
                                continue;
                            },
                        };
                        debug!("{} placed into user hashtable under name {}", irc_event.remote_addr, username);


                    } else {
                        match irc_event.event {
                            Nick(username) => {
                            },
                            User(username,realname) => {
                            }
                            _ => {
                                error!("irc event uninpl in irc event handler");
                                continue;
                            },
                        };
                    };
                }

                endpoint_event = endpoint_event_source.recv() => {
                    let endpoint_event = match endpoint_event {
                        Some(endpoint_event) => endpoint_event,
                        None => {
                            //error!("IRC endpoint {} received invalid event. DISCARDING EVENT", self.name);
                            // TODO 
                            error!("IRC endpoint {} received invalid event. DISCARDING EVENT", "$NAME");
                            continue;
                        }
                    };


                    let response = match endpoint_event {
                        _ => error!("IRC endpoint event handler: unimpl'd event"),
                    };

                    //tracing!("IRC endpoint {} received event", self.name());
                }
            }
            
        }
    }
}

// impl so that our IrcEndpoint complies with the functionalities demanded from
// an EndpointBackend
#[async_trait]
impl EndpointBackend for IrcEndpoint {
    async fn start(mut self: Box<Self>, mut name: String, server_event_sink: mpsc::Sender<ServerEvent>, endpoint_event_source: mpsc::Receiver<EndpointEvent>) -> Result<()> {
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

        self.run(endpoint_event_source).await;
        Ok(())
    }

    //async fn kick_user();
    //async fn ban_user();

    //async fn connected_users()
}


