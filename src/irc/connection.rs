use color_eyre::Result;
use crate::endpoint::EndpointBackend;
use async_trait::async_trait;
use log::{error, info,debug,trace};

use ellidri_tokens::Message as IrcMessage;

use crate::event::EndpointEvent;


use crate::message::Message;
use futures::StreamExt;

use color_eyre::eyre::eyre;

use std::pin::Pin;
use std::net::{SocketAddr};

use tokio::sync::{mpsc, oneshot, RwLock};

use tokio::net::{TcpStream, TcpListener};
use futures::stream::FuturesUnordered;

use tokio::io::AsyncReadExt;

use crate::irc::{event::IrcEvent, user::IrcUser};
use crate::irc::event::IrcEventType;

use std::sync::Arc;

// holds data associated with a single irc client connection to our server
pub struct IrcClientConnection {
    pub associated_user: Arc<RwLock<IrcUser>>,
    pub irc_session_event_sink: mpsc::Sender<IrcEvent>,
}

impl IrcClientConnection {
    async fn irc_message_to_irc_event(message: &IrcMessage<'_>) -> Result<IrcEventType> {
        //let (response_sink, response_source)

        let irc_event = match message.command {
            Ok(cmd) => {
                use ellidri_tokens::Command::*;
                match cmd {
                    Nick => {
                        if message.num_params == 1 {
                            Ok( IrcEventType::Nick(message.params[0].into()) )
                        } else {
                            Err(eyre!("NICK command wrong amount of parameters"))
                        }
                    },
                    _ => Err(eyre!("cmd {} not implemented yet", cmd)),
                }
            },
            Err(custom_cmd) => {
                Err(eyre!("custom cmd {} not implemented yet", custom_cmd))
            },
        };

        irc_event
    }

    // function just generates IrcEvents from the parsed messages and sends them to the
    // IrcEndpointBackend handler. Or translates incomming IrcEvents into irc messages to send to
    // the clients
    pub async fn handle(mut connection: TcpStream, addr: SocketAddr, irc_handler_event_sink: mpsc::Sender<IrcEvent>, mut irc_session_event_source: mpsc::Receiver<IrcEvent>  ) {
        trace!("connection handler for {}", addr);
        // TODO make this user configureable. Though 512 should be the default msg size and 4096
        // should be the max space tags can use in ircv3 by spec

        let mut buf = [0;512 + 4096];
        loop {
            tokio::select! {
                read_result = connection.read(&mut buf) => {
                    let bytes_read = match read_result {
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

                    // TODO can we make this so it never fails? TODO if not handle
                    let irc_event_type = Self::irc_message_to_irc_event(&msg_parsed).await.unwrap();
                    
                    let irc_event = IrcEvent {
                        event: irc_event_type,
                        //response_sink
                    };

                    // TODO this shouldn't fail. And if when and handle ofc
                    irc_handler_event_sink.send(irc_event).await.unwrap();
                }

                irc_session_event = irc_session_event_source.recv() => {
                    let event = match irc_session_event {
                        Some(ev) => ev,
                        None => {
                            error!("received invalid irc session event in irc connection handler");
                            continue;
                        },
                    };
                    // TODO trace unclear
                    trace!("received irc_session_event in irc connection handler");

                    // TODO send response back
                    let response = match event {
                        _ => {},
                    };
                }
            }

        }

        info!("closing connection to {}", addr);
    }
}
