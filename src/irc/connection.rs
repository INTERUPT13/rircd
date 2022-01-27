use color_eyre::{eyre::eyre, Result};
use tokio::sync::mpsc;
use std::{
    sync::Arc,
    net::SocketAddr
};

use crate::irc::event::IrcEvent;

// each plain open connection is represented by this struct. It contains:
// - the address of the connection
// - a way to communicate with the underlaying handler for that connection
struct IrcConnectionPlain {
    client_addr: SocketAddr,
    connection_handler_irc_event_sink: mpsc::Sender<IrcEvent>,
}

// each tls open connection is represented by this struct. It contains:
// - the address of the connection
// - a way to communicate with the underlaying handler for that connection
struct IrcConnectionTls {
    client_addr: SocketAddr,
    connection_handler_irc_event_sink: mpsc::Sender<IrcEvent>,
    //cert: ...,
}


impl IrcConnectionTls {
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


impl IrcConnectionPlain {
    // handler function for an open plain Tcp connection
    pub async fn handle_plain(self: Arc<Self>) {
        panic!("TODO handle()")
    }


    pub async fn init_plain(self: &Arc<Self>) -> Result<Self> {
        // TODO init stuff that can go wrong
        tokio::spawn( self.clone().handle_plain() );

        return Err(eyre!("init_plain_not_impld"));
    }

}
