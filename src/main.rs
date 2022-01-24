#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

#![feature(async_closure)]

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

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use color_eyre::{eyre::eyre, Result};
use async_trait::async_trait;

use tokio::sync::{RwLock,mpsc};
use std::convert::TryFrom;

use std::net::SocketAddr;

// TODO channels can be assigned a the endpoints they shall span accross
// like so its possibl to have a channel that delivers to From irc to Tg and the other way around
// but leaves out for example matrix or includes it whatever ...
trait Channel {
}

trait Pm {
}

trait DisplayTelegramMessage {
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
    user: Arc<dyn User>,

    // TODO on None show the yotosuba 404 pic instead of the image :)
    // with a small notice that its actully missing (4 real) 
    img: Option<Image>,
    // TODO for logging/dbg: msg_raw: 
    text: Option<String>,
}

#[async_trait]
trait User {
    //TODO 
    //! the data User holds such as a nichname is always RwLock<>ed (or atomic in case of counters
    //! or shit)
    //! TODO always use .write() when unsure if the following operation MIGHT/WILL write. For
    //! example when doing changing data after first inspecting it we still write lock it
    //! eventhough we might not write to it in fact. The problem is that if we:
    //! if read_lock().cond { ... write().stuff } we would have to drop the read handle before
    //! getting a write and it can not be guaranteed that we get the write and not something else
    //!  ---> data race
    //!
    //!  thats why all write operations should only be done by the endpoint such as renaming or
    //!  shit

    async fn username(self: &Self) -> String;
    async fn channels(self: &Self) -> Vec<Arc<dyn Channel>>;

    // TODO: Impl
    // send response + 
    //async fn set_username(self: Arc<Self>) -> Result<()>;

}

#[async_trait]
impl User for Arc<IrcUser> {
  async fn username(self: &Self) -> String { String::new() }
  async fn channels(self: &Self) -> Vec<Arc<dyn Channel>> { 
      self.channels.read().await.iter().map(|c| {
          c.clone()
      }).collect()
  }
}

struct IrcUser {
    // TODO type has maxlength or is guaranteed to be max
    nick: RwLock<String>,
    channels: RwLock<Vec<Arc<IrcChannel>>>
}

impl Channel for IrcChannel {
}

struct IrcChannel {
}

struct IrcMessage {
}

struct IrcEndpointConfig {
    name: Option<String>,
    // TODO allow port ranges
    sockaddrs: Vec<SocketAddr>,
}



impl TryFrom<IrcEndpointConfig> for IrcEndpoint {
    type Error =  color_eyre::Report;
    fn try_from(conf: IrcEndpointConfig) -> Result<Self> {
        Ok(IrcEndpoint {
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


    async fn run(mut self: Box<Self>) {
        let (s,r) = mpsc::channel(999); // TODO user defineable 

        println!("{}", self.name());
        // TODO DBG
        if self.name() == "irc_2" {
            self.event_to_server.unwrap().send(EndpointEventInfo {
                event: EndpointEvent::LoadEndpoint(
                    Box::new(IrcEndpoint::try_from( IrcEndpointConfig {
                            name: Some("irc_loaded".into()),
                            sockaddrs: vec!["0.0.0.0:4004".parse().unwrap()]
                        }).unwrap())),

                response_to: s,
            }).await; // TODO remove also it has no await 
        } 

        
        // TODO run server, return failure to started if failed
        let mut event_from_server = self.event_from_server.unwrap();

        // TODO log
        //println!("loaded  endpoint");
        //sleep(Duration::from_millis(3000)).await; // TODO DBG remove
        //println!("sending shutdown event to server");


        loop {
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
                _ = sleep(Duration::from_secs(99000)) => {
                    ()
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
    ShutdownEndpoint,
    ShutdownServer,

    LoadEndpoint(Box<dyn Endpoint  + Send>)
}

impl std::fmt::Debug for Box<dyn Endpoint + Send> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO impl
        Ok(())
    }
}

struct IrcEndpoint {
    //TODO 
    event_to_server: Option<mpsc::Sender<EndpointEventInfo>>,
    event_from_server: Option<mpsc::Receiver<EndpointEventInfo>>,
    flg: usize,

    // TODO ensure name can never contain spaces for simple parsing
    name: Option<String>,
}

#[derive(Debug)]
enum EndpointEventResponse {
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
    // fn send_event(&self, EndpointEventInfo>, timeout) ->  Result<Response,Timeout>
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
        sockaddrs: vec!["0.0.0.0:4000".parse().unwrap()]
    })?;

    let ep2 = IrcEndpoint::try_from( IrcEndpointConfig {
        name: Some("irc_2".into()),
        sockaddrs: vec!["0.0.0.0:4001".parse().unwrap()]
    })?;

    srv.load_endpoint(Box::new(ep1));
    srv.load_endpoint(Box::new(ep2));

    srv.run().await

}
