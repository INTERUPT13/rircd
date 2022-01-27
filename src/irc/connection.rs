use color_eyre::{eyre::eyre, Result};
use tokio::{
    sync::mpsc,
    net::TcpStream
};
use std::{
    sync::Arc,
    net::SocketAddr
};

use crate::irc::event::{IrcEvent,IrcConnectorEventOut};

// each plain open connection is represented by this struct. It contains:
// - the address of the connection
// - a way to communicate with the underlaying handler for that connection
pub struct IrcConnectionHandlePlain {
    pub client_addr: SocketAddr,
    pub connection_handler_irc_event_sink: mpsc::Sender<IrcConnectorEventOut>,
}

// each tls open connection is represented by this struct. It contains:
// - the address of the connection
// - a way to communicate with the underlaying handler for that connection
pub struct IrcConnectionHandleTls {
    client_addr: SocketAddr,
    connection_handler_irc_event_sink: mpsc::Sender<IrcConnectorEventOut>,
    //cert: ...,
}


impl IrcConnectionHandleTls {
    pub async fn init_tls(self: &Arc<Self>/*,cert: CryptShit*/) -> Result<Self> {
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
    async fn handle_plain(self: Arc<Self>) {
        panic!("TODO handle()")
    }


    pub async fn init_plain(self: &Arc<Self>) -> Result<Self> {
        // TODO init stuff that can go wrong
        tokio::spawn( self.clone().handle_plain() );

        return Err(eyre!("init_plain_not_impld"));
    }

}
