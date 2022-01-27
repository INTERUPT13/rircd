use crate::endpoint::{EndpointHandle,EndpointBackend};
use tokio::sync::mpsc;
use color_eyre::Result;
use std::sync::Arc;

use tokio::sync::RwLock;



// Server "state" struct
pub struct Server {
    endpoint_handlers: Vec<EndpointHandle>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            endpoint_handlers: Vec::new(),
        }
    }
}

impl Server {

    // TODO spawn tokio reactor from here. Not from main so we can configure
    // it at startup (st/mt/threads/...)
    pub async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    // start_endpoints -> endpoints scheduled for start
    // TODO this should not be pub
    pub async fn run(&mut self) -> Result<()> {
    //async fn run(&mut self, start_endpoints: Vec<Arc<dyn EndpointBackend>>) -> Result<()> {
        let (server_event_sink,mut server_event_source) = mpsc::channel(99); //TODO cfg

        // TODO spawn endpoints scheduled to start
        
        // DBG 
            self.endpoint_handlers.push( {
                let name = "lol".into();
                let epb =crate::irc::IrcBackendEndpoint::default();
                let epb = Arc::new(epb) as Arc<dyn EndpointBackend + Sync + Send>;

                epb.try_init(name,server_event_sink).await.unwrap()
            });
        //

        //server event loop
        loop {
            let server_event = match server_event_source.recv().await {
                Some(ev) => {
                    //log_trace("server event loop received server_event")
                    ev
                },
                None => {
                    //log_warn("server event loop received server_event=None")
                    continue;
                },
            };
        }

    }
}
