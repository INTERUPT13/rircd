use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;
use crate::event::Event;
use crate::channel::Channel;

// trait that is being fullfilled by the actual protocol specific Endpoints
// like Irc Telegram ....
trait EndpointBackend {
}

struct Endpoint {
    // we track the channels an endpoint is part of so that in case the
    // endpoint gets removed/goes offline we can  first remove it from
    // the channels it is part of. To prevent messages being delivered
    // into nowhere
    in_channels: Vec<Arc<Channel>>,
    backend: Box<dyn EndpointBackend>,
}

// changing internal values of endpoints isn't performed through directly
// altering these but rather trough sending commands to the endpoint. 
// Somewhat like in an Actor framework. This struct represents the contact
// information that can be used to contact an endpoint
pub struct EndpointContact {
    name: String,
    endpoint_event_sink: mpsc::Sender<Event>
}

impl EndpointContact {
}


