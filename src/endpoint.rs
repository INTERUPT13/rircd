use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;
use crate::event::ServerEvent;
use crate::channel::Channel;
use color_eyre::Result;
use log::{debug,info};

use async_trait::async_trait;

// trait that is being fullfilled by the actual protocol specific Endpoints
// like Irc Telegram ....
#[async_trait]
pub trait EndpointBackend {
    async fn start(self: Box<Self>, name: String) -> Result<()>;
}

impl Endpoint {
    // TODO it should be possible to just mut borrow the endpoint
    // since server shouldn't terminate before self.run() or stuff
    pub async fn start(self, event_sink: mpsc::Sender<ServerEvent>) -> Result<()> {
        debug!("trying to start endpoint {}", self.name);
        self.backend.start(self.name).await
    }

    pub fn new(name: String, backend: Box<dyn EndpointBackend>) -> Endpoint {
        Self {
            name,
            backend,
            in_channels: Vec::new(),
        }

    }
}

pub struct Endpoint {
    // we track the channels an endpoint is part of so that in case the
    // endpoint gets removed/goes offline we can  first remove it from
    // the channels it is part of. To prevent messages being delivered
    // into nowhere
    name: String,
    backend: Box<dyn EndpointBackend>,
    in_channels: Vec<Arc<Channel>>,
}

// changing internal values of endpoints isn't performed through directly
// altering these but rather trough sending commands to the endpoint. 
// Somewhat like in an Actor framework. This struct represents the contact
// information that can be used to contact an endpoint
pub struct EndpointContact {
    name: String,
    endpoint_event_sink: mpsc::Sender<ServerEvent>
}

impl EndpointContact {
}


