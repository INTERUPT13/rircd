use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;
use crate::event::{EndpointEvent,ServerEvent,  EndpointEventResponse};
use crate::channel::Channel;
use color_eyre::Result;
use log::{debug,info};
use color_eyre::eyre::eyre;

use async_trait::async_trait;

// trait that is being fullfilled by the actual protocol specific Endpoints
// like Irc Telegram ....
#[async_trait]
pub trait EndpointBackend {
    async fn start(self: Box<Self>, name: String, server_event_sink: mpsc::Sender<ServerEvent>, endpoint_event_source: mpsc::Receiver<EndpointEvent>) -> Result<()>;
}

impl Endpoint {
    // TODO it should be possible to just mut borrow the endpoint
    // since server shouldn't terminate before self.run() or stuff
    pub async fn start(self, server_event_sink: mpsc::Sender<ServerEvent>, endpoint_event_source: mpsc::Receiver<EndpointEvent>) -> Result<()> {
        debug!("trying to start endpoint {}", self.name);
        self.backend.start(self.name, server_event_sink, endpoint_event_source).await
    }

    pub fn new(name: String, backend: Box<dyn EndpointBackend>) -> Endpoint {
        Self {
            name,
            backend,
            in_channels: Vec::new(),
        }

    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Endpoint {
    // we track the channels an endpoint is part of so that in case the
    // endpoint gets removed/goes offline we can  first remove it from
    // the channels it is part of. To prevent messages being delivered
    // into nowhere

    // TODO endpoints have no need to be aware of their names. (WAIT tracing?!) Only 
    // EndpointContact's have. Also pub is just temp   
    pub name: String,
    backend: Box<dyn EndpointBackend>,
    in_channels: Vec<Arc<Channel>>,
    //associated_users: Vec<Arc<User>>
}

// changing internal values of endpoints isn't performed through directly
// altering these but rather trough sending commands to the endpoint. 
// Somewhat like in an Actor framework. This struct represents the contact
// information that can be used to contact an endpoint
//
// TODO use name() and send_event
pub struct EndpointContact {
    pub name: String,
    pub endpoint_event_sink: mpsc::Sender<EndpointEvent>
}

impl EndpointContact {
    async fn name(&self) -> String {
        self.name.clone()
    }
    async fn send_event(&self, event: EndpointEvent) -> Result<EndpointEventResponse> {
        Err(eyre!("TODO"))
    }
}


