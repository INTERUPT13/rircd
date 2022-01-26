use color_eyre::Result;
use crate::endpoint::EndpointBackend;
use async_trait::async_trait;
use log::{info,debug,trace};

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

use crate::irc::user::IrcUser;

use std::sync::Arc;

// holds data associated with a single irc client connection to our server
pub struct IrcClientConnection {
    associated_user: Arc<RwLock<IrcUser>>,
    event_sink: mpsc::Sender<EndpointEvent>,
}

impl IrcClientConnection {
    async fn irc_message_to_rircd_message(message: &IrcMessage<'_>) -> Result<Message> {
        Err(eyre!(""))
    }

    pub async fn handle(mut connection: TcpStream, addr: SocketAddr, event_source: mpsc::Receiver<EndpointEvent>) {
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

            let ircd_msg = Self::irc_message_to_rircd_message(&msg_parsed).await;


        }

        info!("closing connection to {}", addr);
    }
}
