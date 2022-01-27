use async_trait::async_trait;
use tokio::{
    net::TcpListener,
    sync::{mpsc,RwLock}
};

use crate::endpoint::{EndpointBackend,EndpointHandle};
use color_eyre::{eyre::eyre, Result};

use std::{
    sync::Arc,
    net::SocketAddr,
    collections::HashMap
};
use crate::{
    event::{ServerEvent,EndpointBackendEvent},
    irc::{
        event::IrcEvent,
        user::IrcUser,
        channel::IrcChannel
    }
};

impl Default for IrcBackendEndpoint {
    fn default() -> Self {
        Self {
            associated_users: RwLock::new(HashMap::new()),
            associated_channels: RwLock::new(HashMap::new()),
            plain_bind_addrs: RwLock::new(Vec::new()),
            tls_bind_addrs: RwLock::new(Vec::new()),
        }
    }
}


// -intended to be shared between all the IrcConnection::handle()'rs
// -a single backend_event_handle()'r is responsible for receiving
// 
struct IrcBackendEndpoint {
    // for looking up users by ip:port connection or
    // just enumerating them
    associated_users: RwLock<HashMap<SocketAddr, Arc<IrcUser>>>,
    // for looking up channels by their Channel name.
    // or just enumerating them
    associated_channels: RwLock<HashMap<String, Arc<IrcChannel>>>,


    // keep track of the addrs the Endpoint is currently binding to
    // this is mostly to be used to fill in user perfored querys
    plain_bind_addrs: RwLock<Vec<SocketAddr>>,
    tls_bind_addrs: RwLock<Vec<SocketAddr>>,
}

#[async_trait]
impl EndpointBackend for IrcBackendEndpoint {
    async fn try_run(
        self: Arc<Self>,
        name: Arc<RwLock<String>>,
        server_event_sink: mpsc::Sender<ServerEvent>,
        endpoint_backend_event_source: mpsc::Receiver<EndpointBackendEvent>) -> Result<()> {
        
        // do stuff that might fail such as socket alloc
        let sockets = Vec::new();

        tokio::spawn( self.handle(sockets,server_event_sink,endpoint_backend_event_source) );
        Ok(())
    }
}

impl IrcBackendEndpoint {
    pub async fn lookup_username(username: String) -> Result<IrcUser> {
        Err(eyre!("not impld"))
    }

    pub async fn lookup_channel(channel_name: String) -> Result<IrcChannel> {
        Err(eyre!("not impld"))
    }


    // these functions copy data so they are only meant to be used to display
    // stuff for example on ircd_server commands from the shell or shitgg
    pub async fn list_users(channel_name: String) -> Result<Vec<IrcChannel>> {
        Err(eyre!("not impld"))
    }

    pub async fn lookup_channels(channel_name: String) -> Result<IrcChannel> {
        Err(eyre!("not impld"))
    }

    pub async fn handle(
        self: Arc<Self>,
        sockets: Vec<TcpListener>,
        server_event_sink: mpsc::Sender<ServerEvent>,
        endpoint_backend_event_source: mpsc::Receiver<EndpointBackendEvent>) -> Result<EndpointHandle> {

        // spawn off acceptors for each socket (with a channel)
        //  > these spawn of IrcConnecton::handler()'s (passing the channel along)
        //    > there receive irc messages from the socket, parse them and send them trough the channel
        //    > there receive irc messages from the channel, turn them into irc messages and send them to the socket

        // tokio::select! endpoint_backend_event_source, acceptor_e
        Err(eyre!("not implid"))
    }
}
