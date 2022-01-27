use async_trait::async_trait;
use std::sync::Arc;
use color_eyre::Result;
use tokio::sync::{RwLock, mpsc};
use crate::event::{EndpointBackendEvent,ServerEvent};

impl EndpointHandle {
    // function that is responsible 
    pub fn get_config(name: String) -> Result<()> {
        //TODO
        Ok(())
    }
    //pub fn get_name(name: String) -> Result<String> {
    //    //TODO
    //    Ok(())
    //}

    pub fn change_name(new_name: String) -> Result<()> {
        //TODO
        Ok(())
    }
}

// when an EndpointBackend is spawned this handle is returned. It can be used
// to identify the endpoint by name and communicate with it
pub struct EndpointHandle {
    // Arc'd/RwLock'd since the string is shared with the EndpointBackend handler
    // so that name changes  will be synced. The EndpointBackend needs to know
    // its own name for tracing
    name: Arc<RwLock<String>>,
    endpoint_backend_event_sink: mpsc::Sender<EndpointBackendEvent>,
}


#[async_trait]
pub trait EndpointBackend {
    // tries to start the EndpointBackend. Returns an error in case allocation
    // of the EndpointBackend wasn't possible. For example socket binding
    // is a thing that might fail
    //
    // returns an EndpointContact so its possible for the server to communicate
    // with the EndpointBackend
    //
    // server_event_sink provides a channel for the EndpointBackend handler to send events
    // to the server.
    async fn try_init(
        self: Arc<Self>,
        name: String,
        server_event_sink: mpsc::Sender<ServerEvent>,
        endpoint_backend_event_source: mpsc::Receiver<EndpointBackendEvent>) -> Result<EndpointHandle> where Arc<Self>: Send{

            //name is Arc<RwLocked<<>>'d so it can be shared -> changes are synced
            let name = Arc::new(RwLock::new(name));

            //create channel so that the server can contact the endpoint
            let (endpoint_backend_event_sink,endpoint_backend_event_source) = mpsc::channel(99); // TODO cfg

            self.try_run(name.clone(), server_event_sink,endpoint_backend_event_source).await?;

            Ok(EndpointHandle {
                name,
                endpoint_backend_event_sink,
            })
    }

    async fn try_run(
        self: Arc<Self>,
        name: Arc<RwLock<String>>,
        server_event_sink: mpsc::Sender<ServerEvent>,
        endpoint_backend_event_source: mpsc::Receiver<EndpointBackendEvent>) -> Result<()>;
}
