#![allow(where_clauses_object_safety)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

#![feature(async_closure)]
#![feature(generators)]

// TODO when routing messages (default -> Msg goes to every endpoint except the sender) but it should
// be possible to send them to only selected endpoints  as  some sort of stealth/shadow delivery

// TODO remove unneccescary trait bounds
// 
// TODO I got the suspicion that mpsc.recv() will yield also if the channel  sender end is dropped
// which uhh well we gotta be aware of that or make sure it wont happen
// [the compiler shouldn't drop it if the code still uses it in some way later]
//
//TODO endpoint keepalive checks
//TODO event.send() timeout behavior (otherwise we risk hangs)
//TODO for responses use single prod single consumer channels instead of mpsc
//
// TODO rename things for more clearity like:
// -the channels recv/send ends/fields
// - eg EndpointEvent(Info/Response) -> Event / EventResponse

use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

use tokio::time::{self, Duration};
use tokio::time::sleep;

use async_stream::stream;
use tokio::net::*;
use tokio::io::AsyncReadExt;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use color_eyre::{eyre::eyre, Result};
use async_trait::async_trait;

use tokio::sync::{Mutex, RwLock,mpsc};
use std::convert::TryFrom;

use std::net::SocketAddr;

// TODO channels can be assigned a the endpoints they shall span accross
// like so its possibl to have a channel that delivers to From irc to Tg and the other way around
// but leaves out for example matrix or includes it whatever ...

// the Channel abstraction is some sort of global server wide channel that can span accross
// multiple endpoints of possible different types
struct Channel {
    name: String,
    endpoints: Vec<Arc<EndpointInfo>>,
}

impl Channel {
    //TODO result -> statuses
    //
    //TODO V 
    //fn send_msg_to_endpoints_no_echo(&self, msg: &Arc<Message>) -> Result<()> {
    //    self.send_msg_to_endpoints
    //}
    async fn send_msg_to_endpoints(&self, msg: &Arc<Message>) -> Result<()> {
        //TODO use futures unordered
        let mut results = FuturesUnordered::new();

        // TODO this should never happen (should it? can it?)
        assert!( self.endpoints.len() != 0);

        self.endpoints.iter().for_each(|ep| {
            // TODO userdef cfg.timeout
            println!("sending msg to {}", ep.name);
            results.push( ep.send_event(EndpointEvent::Message(msg.clone()), Duration::from_secs(10)))
        });

        // we gotta use map as this gives us ownership of res which avoids a lifetime issue
        let errmsg: String = results.filter_map(|mut res| async move {
            match res {
                Ok(EndpointEventResponse::MessageDeliverySuccess) => None,
                _ => Some(res),
            }
        }).map(|e| match e {
            Err(e) => format!("{}\n", e),
            Ok(res) => format!("{:?}", res)
        }).collect().await;

        //TODO is it possible for this thing to spit out an Option<>
        if errmsg.is_empty() {
            Ok(())
        } else {
            Err(eyre!(errmsg))
        }
    }
}



struct TelegramMessage {
}

enum DeliveryMode {
    Channel,
    PrivateMessage,
    BroadcastChannel, //(deliver message to every channel)
    BroadcastUse, //(deliver message to every user)
    SerdeError      //server commands and such
}

// - actually I discarded that idea for now because in most cases
// the url will still be fetched (reuploads + analysis)[optional]
//enum ImageType {
//    Picture(RustImgLibType), //just a pic
//    Sticker(RustImgLibType),
//    Url(String), // for irc people 
//}

// TODO order them by preference or even better let the user define the
// prefered ordering to which format it should be converted to preferilby
// eg if we like good quality pics we always encode BMP's to PNG's if 
enum ImageFormat {
    PNG, // PNG(Arc[u8])
    JPG,
    BMP,
}

// TODO user config for always trying to strip metadata from images for
// extra priv protection
impl Image {
    // if the endpoint accepts a format and its in that form originally
    // why convert it whey you could just send whats already ther
    //
    fn original_format(&self) -> ImageFormat {
        ImageFormat::BMP
    }

    // TODO make sure to always use cached images
    //fn to_png(&self) {
    // if self.png_cached.read() {
    //  *self.png_cached.write() = PNG::From(self.orig)
    // } else 
    //   self.png_cached().read()
    // }
}

struct Image {
    original: ImageFormat,
    //png_cached: RwLock< Option<Arc<[u8]>> >,
    // .. TODO more cached
}



// I chose to go with a struct instead of traits since not That
// much flexibility is needed here over extra code
//
// eg implementing Into<> for each dyn Message to each struct *Message
// would be more code than just parsing it into a struct Message
struct Message {
    delivery_mode: DeliveryMode,
    endpoint_info: Arc<EndpointInfo>,
    
    sending_user: Arc<User>,
    channel: Option<Arc<Channel>>,

    // TODO on None show the yotosuba 404 pic instead of the image :)
    // with a small notice that its actully missing (4 real) 
    img: Option<Image>,
    // TODO for logging/dbg: msg_raw: 
    text: Option<String>,
}

// server wide abstraction for a user
struct User {
    name: String,
    // TODO should this be a vec so a user can have the same identity on multiple 
    // endpoints or would that just fuck up things and have no point
    endpoint: Arc<EndpointInfo>,
    //TOOD 
    //prems: Vec3<enum perm>,
    //groups: Vec3<Arc<Group>>
}


struct IrcUser {
    // TODO type has maxlength or is guaranteed to be max
    nick: RwLock<String>,
    channels: RwLock<Vec<Arc<IrcChannel>>>
}


struct IrcChannel {
    motd: Option<String>,
    name: String,
}

struct IrcMessage {
}

struct IrcEndpointConfig {
    name: Option<String>,
    // TODO allow port ranges
    bind_sockaddrs: Vec<SocketAddr>,
}



impl TryFrom<IrcEndpointConfig> for IrcEndpoint {
    type Error =  color_eyre::Report;
    fn try_from(conf: IrcEndpointConfig) -> Result<Self> {
        Ok(IrcEndpoint {
            listeners_plain: Vec::new(),
            bind_sockaddrs: conf.bind_sockaddrs,
            event_to_server: None,
            event_from_server: None,
            flg: 1,
            name: conf.name,
        })
    }
}


struct MsgIrc {
}

#[async_trait]
trait Endpoint {
    fn link_event_to_server(&mut self, event_to_server: mpsc::Sender<EndpointEventInfo>);
    fn link_event_from_server(&mut self, event_to_server: mpsc::Receiver<EndpointEventInfo>);

    fn load(self: Box<Self>) -> Result<()> where Self: Send  + 'static{
        println!("attepting to load endpoint ..."); //TODO log
        tokio::spawn(  self.run() );
        // wait for response & return it
        Ok(())
    }
    async fn run(self: Box<Self>) where Self: Send;


    fn name(&self) -> String;
}

impl IrcEndpoint {
    async fn init_plain(&mut self) -> Result<Vec<TcpListener>> {
        // TODO use "stream!" or something to make the init more async -> speedz
        let mut listeners_plain = Vec::new();

        for addr in &self.bind_sockaddrs {
            // TODO logging
            listeners_plain.push( TcpListener::bind(&addr).await? );
        }
        Ok(listeners_plain)
    }
}

#[async_trait]
impl Endpoint for IrcEndpoint {
    fn name(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else {
            "IRC($ADDR,$PORT)".into()
            // "OR IRC({})
            //      hash( [ BIND_IPS ][ BIND_PORTS] )   <maybe even a port range :o>
        }
    }

    // TODO fn set_name() 
    // > force no spaces for simple parsing

    fn link_event_to_server(&mut self, event_to_server: mpsc::Sender<EndpointEventInfo>) {
        //self.event_to_server = event_to_server;
        self.event_to_server = Some(event_to_server);
    }
    fn link_event_from_server(&mut self, event_from_server: mpsc::Receiver<EndpointEventInfo>) {
        //self.event_to_server = event_to_server;
        self.event_from_server = Some(event_from_server);
    }



    async fn run(mut self: Box<Self>) where Self: Send{
        // TODO 
        let cfg_tls = false;
        // TODO run server, return failure to started if failed
        let listeners_plain = self.init_plain().await.unwrap();
        // TODO 
        let mut event_from_server = self.event_from_server.unwrap();

        // TODO log
        //println!("loaded  endpoint");
        //sleep(Duration::from_millis(3000)).await; // TODO DBG remove
        //println!("sending shutdown event to server");

        let mut open_connections = FuturesUnordered::new();



        let mut s = String::new();

        loop {
            // TODO can theis be don prettier?
            let mut new_connections = {
                let mut f = FuturesUnordered::new();
                listeners_plain.iter().for_each(|l| {
                    f.push(l.accept());
                });
                f
            };


            tokio::select! {
                //TODO this should be safe to unwrap() since it has to be initalized at this
                //point ?!
                event = event_from_server.recv() => {
                    if let Some(event) = event {
                        match event.event {
                            EndpointEvent::ShutdownEndpoint =>  {
                                println!("endpoint received shutdown signal");
                                event.response_to.send( EndpointEventResponse::EndpointShutdownSuccess ).await;
                                break;
                            }
                            _ => { println!("endpoint received uninplemented Event"); }// TODO log
                        };
                    } else {
                        // TODO this shouldn't happen in the first place and if it does
                        // at least idk give a CRIT warning and tell the user where
                        println!("couldn't recv() uuh wft ?! :( ");
                    };
                }
                // TODO
                // pkg => tls.recv()
                // pkg => plain.recv()
                // TODO we cant have the sleep here it makes select behave weird

                // accept new connection
                conn = new_connections.next() => {
                    println!("{:?}", conn);
                    // TODO handle + log errors
                    let mut l = conn.unwrap().unwrap();
                    let mut l = Box::leak(Box::new(l.0));
                    let mut l = l.read_to_string(&mut s);
                    open_connections.push( l );
                }

                pack = open_connections.next() => {
                    
                    println!("{}", s);
                }
            };
        }
        // TODO close socket or shit
    }
}


// message format used when displayed to internal log
struct MsgLog {
}


trait Msg {
    // Into<MsgIrc>
    // Into<MsgTg>
    // ...

    // Into<MsgLog>
}

#[derive(Debug)]
enum EndpointEvent {
    Message(Arc<Message>),

    ShutdownEndpoint,
    ShutdownServer,

    LoadEndpoint(Box<dyn Endpoint  + Send>)
}


impl std::fmt::Debug for Message  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO impl
        Ok(())
    }
}

impl std::fmt::Debug for Box<dyn Endpoint + Send> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO impl
        Ok(())
    }
}

struct IrcEndpoint {
    listeners_plain: Vec<TcpListener>,
    bind_sockaddrs: Vec<SocketAddr>,
    //TODO 
    event_to_server: Option<mpsc::Sender<EndpointEventInfo>>,
    event_from_server: Option<mpsc::Receiver<EndpointEventInfo>>,
    flg: usize,

    // TODO ensure name can never contain spaces for simple parsing
    name: Option<String>,
}

#[derive(Debug)]
enum EndpointEventResponse {
    MessageDeliverySuccess,
    EndpointShutdownSuccess
}


struct EndpointEventInfo where mpsc::Sender<EndpointEventResponse>: Send{
    response_to: mpsc::Sender<EndpointEventResponse>,
    event: EndpointEvent,
    // TODO
    //sender: String,
    //reason: String,
}


//TODO impl and use

impl EndpointInfo {
    // TODO use and impl
    async fn send_event(&self, ev: EndpointEvent, timeout: Duration) ->  Result<EndpointEventResponse,tokio::time::error::Elapsed> {
        let (s,mut r) = mpsc::channel(999); // TODO user def 
        let evi = EndpointEventInfo {
            response_to: s,
            event: ev,
        };
        self.event_to_endpoint.send(evi);
        //TODO dont unwrap
        time::timeout(timeout, r.recv()).await.map(|res| res.unwrap())
    }
}

struct EndpointInfo {
    name: String,
    event_to_endpoint: mpsc::Sender<EndpointEventInfo>,
}

struct Server {
    endpoint_infos: Vec<Arc<EndpointInfo>>,
    event_sender: mpsc::Sender<EndpointEventInfo>,
    event_receiver: mpsc::Receiver<EndpointEventInfo>
}

impl Server {
    // its guaranteed that there is just one (name is unique). For a more
    // forgiving search use get_endpoint_infos_by_name or sth which uses smart text search TODO
    async fn get_endpoint_info_by_name(&self, name: String) -> Option<Arc<EndpointInfo>> {
        self.endpoint_infos.iter().find_map(|ep| {
            if ep.name == name {
                Some(ep.clone())
            } else {
                None
            }

        })
    }


    //async fn get_endpoint_infos_by_name(name: String) -> Arc<EndpointInfo> { smart_text_search()
    //}
    //async fn get_endpoint_infos_by_id(name: String) -> Arc<EndpointInfo> {};


    fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::channel(999); //TODO runtime configurable size
        Server {
            endpoint_infos: Vec::new(),
            event_sender,
            event_receiver
        }
    }

    fn load_endpoint(&mut self, mut endpoint: Box<dyn Endpoint + Send>) -> Result<()> {
        let (event_sender, event_receiver) = mpsc::channel(999); //TODO runtime cfg bufsz

        endpoint.link_event_from_server(event_receiver);
        endpoint.link_event_to_server(self.event_sender.clone());

        self.endpoint_infos.push( Arc::new( EndpointInfo {
            name: endpoint.name(),
            event_to_endpoint: event_sender,
        }));
        Endpoint::load(endpoint);
        Ok(())
    }

    async fn run(mut self) -> Result<()> {
        let mut running = true;
        println!("started server"); //TODO log
        while running {
            if let Some(event) = self.event_receiver.recv().await {
                // TODO log(HIGH_VERB,{:?}, event) 
                let response = match event.event {
                    EndpointEvent::Message(msg) => {
                        // TODO we will get echo because with for example telegram
                        // endpoints the people in the chan already saw the message as it first
                        // goes to telegram servers then to us. But for irc the others only see it
                        // if they actually get it send by us since we are the server so one gotta
                        // distingulish
                        let echo = true; // TODO read from msg.endpoint.type

                        println!("<{}> [{}] {}", msg.sending_user.name,
                            {
                                &msg.channel.as_ref().unwrap().name
                            },
                            msg.text.as_ref().unwrap());
                        let res = match msg.delivery_mode {
                            DeliveryMode::Channel => {
                                //TODO
                                if !echo {
                                } else {
                                    msg.channel.as_ref().unwrap().send_msg_to_endpoints(
                                        &msg
                                    ).await;
                                }
                            },
                            _ => {
                                panic!("unimp dev mode");
                            },
                        };
                        // TODO return res
                        Ok(())
                    },
                    EndpointEvent::ShutdownServer => {
                        println!("lol shuttin down");
                        running = false;
                        // TODO send ShutdownEndpoint to all endpoints and wait for replies
                        // bubble up Err()'s that might happen and await on a timeout as well
                        // just in case one of the endpoints froze or shit [eg 2nd tok::select
                        // branch

                        let mut shutdown_responses = FuturesUnordered::new();

                        for endpoint_info in &self.endpoint_infos {
                            // TODO names misleading
                            let (res_sender, mut res_recv) = mpsc::channel(999); // TODO 999 -> runtime.cfg
                            let event_info = EndpointEventInfo {
                                event: EndpointEvent::ShutdownEndpoint,
                                response_to: res_sender
                            };
                            // TODO make and use a trait function send_all() instead
                            endpoint_info.event_to_endpoint.send(event_info).await;
                            shutdown_responses.push(async move {

                                // TODO runtime cfg.timeout
                                ( endpoint_info.name.clone(), time::timeout(Duration::from_secs(5), res_recv.recv()).await)
                            })
                        }

                        let timeout_endpoints: Vec<(String,Result<_, tokio::time::error::Elapsed>)> = shutdown_responses.collect().await;

                        for ep in timeout_endpoints {
                            let endpoint_name = ep.0;
                            match ep.1 {
                                Err(elapsed) => {
                                    // TODO log
                                    println!("SHUTDOWN FAIL for \"{}\" reason: timeout", endpoint_name);
                                },
                                Ok(Some(resp)) => {
                                    println!("SHUTDOWN SUCCESS for \"{}\"", endpoint_name);
                                },
                                Ok(None) => {
                                    println!("empty response ?! WFT");
                                },

                            }
                        }

                        Ok(())
                    },

                    EndpointEvent::LoadEndpoint(ep) => {
                        self.load_endpoint(ep);
                        Ok(())
                    },

                    _ => {
                        Err(eyre!("event not implemented"))
                    }
                };
            } else {
                println!("error receiving event TODO log");
            };
        };
        Ok(()) // TODO print detailed reason for shutdown and origin
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let mut srv = Server::new();

    let ep1 = IrcEndpoint::try_from( IrcEndpointConfig {
        name: Some("irc_1".into()),
        bind_sockaddrs: vec!["0.0.0.0:4000".parse().unwrap()]
    })?;

    let ep2 = IrcEndpoint::try_from( IrcEndpointConfig {
        name: Some("irc_2".into()),
        bind_sockaddrs: vec!["0.0.0.0:4001".parse().unwrap()]
    })?;

    let ep3 = IrcEndpoint::try_from( IrcEndpointConfig {
        name: Some("irc_3".into()),
        bind_sockaddrs: vec!["0.0.0.0:4002".parse().unwrap()]
    })?;

    srv.load_endpoint(Box::new(ep1));
    srv.load_endpoint(Box::new(ep2));
    srv.load_endpoint(Box::new(ep3));

    srv.run().await

}











