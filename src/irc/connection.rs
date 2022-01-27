use color_eyre::{eyre::eyre, Result};
use tokio::{
    sync::{mpsc,RwLock},
    net::TcpStream
};
use std::{
    sync::Arc,
    net::SocketAddr
};

use crate::irc::event::{IrcEvent,IrcConnectorEventOut,IrcConnectorEventIn};

// each plain open connection is represented by this struct. It contains:
// - the address of the connection
// - a way to communicate with the underlaying handler for that connection
pub struct IrcConnectionHandlePlain {
    pub client_addr: SocketAddr,
    pub connection_handler_irc_event_sink: mpsc::Sender<IrcConnectorEventOut>,
    pub connection_handler_irc_event_source: mpsc::Receiver<IrcConnectorEventIn>,
}

// each tls open connection is represented by this struct. It contains:
// - the address of the connection
// - a way to communicate with the underlaying handler for that connection
pub struct IrcConnectionHandleTls {
    pub client_addr: SocketAddr,
    pub connection_handler_irc_event_sink: mpsc::Sender<IrcConnectorEventOut>,
    pub connection_handler_irc_event_source: mpsc::Receiver<IrcConnectorEventIn>,
    //cert: ...,
}


impl IrcConnectionHandleTls {
    pub async fn init_tls(self: &Arc<Self>/*,cert: CryptShit*/) -> Result<()> {
        // TODO init stuff that can go wrong
        tokio::spawn( self.clone().handle_tls() );

        return Err(eyre!("init_tls_not_impld"));
    }

    // handler function for an open tls Tcp connection
    pub async fn handle_tls(self: Arc<Self>/*,cert: CryptShit*/) {
        panic!("TODO handle()")
    }
}


impl IrcConnectionHandlePlain {

    // handler function for an open plain Tcp connection
    async fn handle_plain(self, name: Arc<RwLock<String>>) {
        panic!("TODO handle()")
    }


    pub async fn init_plain(self, name: Arc<RwLock<String>>) -> Result<()> {
        // TODO init stuff that can go wrong
        tokio::spawn( self.handle_plain(name) );

        Ok(())
    }

}
