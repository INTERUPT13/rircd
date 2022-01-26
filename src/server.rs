use std::sync::Arc;

use crate::endpoint::EndpointContact;


impl Server {
    pub fn new() -> Server {
        Server {
            endpoints: Vec::new(),
        }
    }
}

pub struct Server {
    endpoints: Vec<Arc<EndpointContact>>,
}
