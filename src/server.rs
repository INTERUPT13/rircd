use crate::endpoint::EndpointHandle;
use tokio::sync::mpsc;
use color_eyre::Result;





// Server "state" struct
struct Server {
    endpoint_handlers: Vec<EndpointHandle>,
}

impl Server {
    // TODO spawn tokio reactor from here. Not from main so we can configure
    // it at startup (st/mt/threads/...)
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        let (server_event_sink,mut server_event_source) = mpsc::channel(99); //TODO cfg

        // TODO spawn endpoints scheduled to start
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
