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
        event::{IrcEvent,
        IrcConnectorEventIn,IrcConnectorEventOut,
        IrcAcceptorEventIn,IrcAcceptorEventOut},
        user::IrcUser,
        channel::IrcChannel,
        connection::{IrcConnectionHandlePlain,IrcConnectionHandleTls}
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
pub struct IrcBackendEndpoint {
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

// holds channels in both directions. So from our IrcEndpoint::handle()'r to
// the acceptor for a given socket and from the acceptor 
// to the IrcEndpoint::handle()r [so its duplex]. Type introduced for clearity
// TODO move in irc/acceptor.rs
struct AcceptorChannelPair {
    endpoint_handler_sink: mpsc::Sender<IrcAcceptorEventOut>,
    endpoint_handler_source: mpsc::Receiver<IrcAcceptorEventIn>,
}

// holds channels in both directions. So from our IrcEndpoint::handle()'r to
// the connection handler for a given connection and from the connection handler
// to the IrcEndpoint::handle()r [so its duplex]. Type introduced for clearity
// TODO move in irc/connection.rs
struct ConnectionHandlerChannelPair {
    endpoint_handler_sink: mpsc::Sender<IrcConnectorEventOut>,
    endpoint_handler_source: mpsc::Receiver<IrcConnectorEventIn>,
}

#[async_trait]
impl EndpointBackend for IrcBackendEndpoint {
    async fn try_run(
        self: Arc<Self>,
        name: Arc<RwLock<String>>,
        server_event_sink: mpsc::Sender<ServerEvent>,
        endpoint_backend_event_source: mpsc::Receiver<EndpointBackendEvent>) -> Result<()> {
        

        // do stuff that might fail such as socket alloc
        let sockets_plain = Vec::new();
        let sockets_tls   = Vec::new();

        tokio::spawn( self.handle(name, sockets_plain, sockets_tls, server_event_sink,endpoint_backend_event_source) );
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

    async fn handle(
        self: Arc<Self>,
        name: Arc<RwLock<String>>,
        listener_plain: Vec<(SocketAddr,TcpListener)>,
        listener_tls: Vec<(SocketAddr,TcpListener)>,
        server_event_sink: mpsc::Sender<ServerEvent>,
        mut endpoint_backend_event_source: mpsc::Receiver<EndpointBackendEvent>) {

        // TODO this part can be quite confusing. Work on that with good naming and
        // encapsulating structures

        //log_debug!("irc backend endpoint handle for \"{}\" spawned", name.read().await);

        //let (connector_event_sink, connector_event_source) = mpsc::channel(99); //TODO cfg

        // acceptor -> endpoint_handler
        // endpoint_handler <- acceptor
        //                          |------- connector events -> endpoint_event_handler
        //                          V
        let (acceptor_event_sink, mut acceptors_event_source) = mpsc::channel(99); // TODO cfg

        //
        // connector -> endpoint_handler
        // endpoint_handler <- connector
        //                          |------- connector events -> endpoint_event_handler
        //                          V
        let (connector_event_sink, mut connectors_event_source) = mpsc::channel(99); // TODO cfg


        //hashmap that resolves an binding address to a channel_sink to send IrcAcceptorEvents to
        let mut acceptor_sinks: HashMap<SocketAddr,mpsc::Sender<IrcAcceptorEventIn>> = HashMap::new();

        //hashmap that resolves the addr:port of an connected peer *ip:port of whom connected*
        //to an channel_sink to send IrcConnectorEvents to
        let connector_sinks: HashMap<SocketAddr, mpsc::Sender<IrcConnectorEventIn>> = HashMap::new();
        // Arc'd/RwLocked so that we can access this from all the 
        let connector_sinks = Arc::new(RwLock::new(connector_sinks));

        {
            let sock_addr = "0.0.0.0:7000".parse::<SocketAddr>().unwrap();
            let sock = TcpListener::bind(sock_addr).await.unwrap();

            // endpoint -> acceptor
            // acceptor <- endpoint
            let (endpoint_to_acceptor_sink, mut endpoint_to_acceptor_source) = mpsc::channel(99); // TODO cfg

            // debug! inserted(sock_addr) into acceptor_sinks_hashmap
            acceptor_sinks.insert(sock_addr, endpoint_to_acceptor_sink);

            // 
            let acceptor_channel_pair = AcceptorChannelPair {
                endpoint_handler_sink: acceptor_event_sink.clone(),
                endpoint_handler_source: endpoint_to_acceptor_source,
            };

            //connector_sinks.insert(
            
            // TODO track acceptors as well. So we can shut them down later on
            tokio::spawn( self.acceptor_plain(name, sock, acceptor_channel_pair, connector_sinks.clone(), connector_event_sink.clone()) );
        }


        // spawn off acceptors for each socket (with a channel)
        //  > these spawn of IrcConnecton::handler()'s (passing the channel along)
        //    > there receive irc messages from the socket, parse them and send them trough the channel
        //    > there receive irc messages from the channel, turn them into irc messages and send them to the socket


        loop {
            tokio::select! {
                acceptor_event = acceptors_event_source.recv() => {
                    println!("acceptor event IN");
                }

                //connector_events
            }
        }
    }

    // acceptor thread for a specific plain socket. Accepts all incomming connections and
    // spawns off an IrcConnecton::handle_plain() handler that handles the connection from now on
    async fn acceptor_plain(
        self: Arc<Self>,
        name: Arc<RwLock<String>>,
        mut socket: TcpListener,
        mut acceptor_channel_pair: AcceptorChannelPair,
        mut connector_sinks: Arc<RwLock<HashMap<SocketAddr,mpsc::Sender<IrcConnectorEventIn>>>>,
        mut connector_to_endpoint_handler: mpsc::Sender<IrcConnectorEventOut>) {
        

        println!("acceptor()");

        loop {
            tokio::select! {
                // from endpoint handler
                acceptor_event_in = acceptor_channel_pair.endpoint_handler_source.recv() => {
                    println!("plain_acceptor {} got acceptor event from endpoint_handler",
                        name.read().await);
                }

                accepted_connection = socket.accept() => {
                    println!("plain acceptor {} got new connection",
                        name.read().await);
                    let connection = match accepted_connection {
                        Ok(c) => c,
                        Err(e) => {
                            //log_debug!(e)
                            continue;
                        },
                    };
                    


                    let connection_handle = IrcConnectionHandlePlain {
                        client_addr: connection.1,
                        connection_handler_irc_event_sink: connector_to_endpoint_handler.clone(),
                    };
                }
            }
        }

    }
}









