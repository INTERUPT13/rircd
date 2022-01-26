use std::sync::Arc;
use color_eyre::{eyre::eyre, Result};
use futures::StreamExt;
use crate::endpoint::{Endpoint, EndpointContact};
use futures::stream::FuturesUnordered;
use tokio::sync::mpsc;

use crate::event::{ServerEventType, ServerEvent, EndpointEvent, EndpointEventType};
use log::{error,info,debug,trace};

struct ServerConfig {
    start_min_endpoints: usize,
}

impl  Default for ServerConfig {
    fn default() -> Self {
        Self {
            start_min_endpoints: 1,
        }
    }
}

impl Server {
    pub fn new() -> Server {
        let (s,r) = mpsc::channel(512); //TODO configureable
        
        // TODO make this a #[test]
        //let rr = s.send(Event {
        //    event: EventType::ShutdownServer("test".into()),
        //    response_sink: tokio::sync::oneshot::channel().0,
        //});
        //
        //futures::executor::block_on(rr);

        Server {
            event_sink: s,
            event_source: r,
            config: ServerConfig::default(),
            endpoints: Vec::new(),
        }
    }

    async fn try_start_all_endpoints(&mut self, endpoints_to_start: Vec<Endpoint>) -> Vec<Result<()>>  {
        // TODO is there a better way?
        let results: FuturesUnordered<_> = endpoints_to_start.into_iter()
            .map(|ep| ep.start(self.event_sink.clone())).collect();
        results.collect().await
    }

    //pub fn start_endpoint(&self);

    pub async fn run(&mut self, endpoints_to_start: Vec<Endpoint>) -> Result<()> {
        info!("starting server");
        {
            debug!("bringing up endpoints");
            let endpoints_status  = self.try_start_all_endpoints(endpoints_to_start).await;
            let endpoints_success: Vec<_> = endpoints_status.iter().filter(|ep| ep.is_ok()).collect();
            if endpoints_success.len() < self.config.start_min_endpoints {
                return Err(eyre!("less than min({}) endpoints started. Aborting",self.config.start_min_endpoints))
            }
            info!("succesfully started {} endpoints", endpoints_success.len());
        }

        debug!("entering event loop");
        loop {
            // do we need select! event. I don't think in this event there will be a second source
            // for futures
            tokio::select! {
                event = self.event_source.recv() => {
                    if event.is_none() {
                        //TODO if cfg.panic_on  { shutdown_panic() }
                        error!("received invalid event in servers event loop.\
                            [this should never happen] -> DISCARDING EVENT");
                        continue;
                    };
                    let event = event.unwrap();

                    debug!("received event {:?}", event);
                    let response = match event.event {
                        ServerEventType::ShutdownServer(reason) => {
                            info!("server shutdown request receiverd. Reason: \"{}\"", reason);
                            // TODO Option<>al
                            //  cfg.broadcast_channels []
                            //  cfg.broadcast_channels_all
                            //  cfg.broadcast_users []
                            //  cfg.broadcast_users_all []
                            break;
                        },

                        ServerEventType::Message(msg) => {
                            debug!("server eventloop received message {:?}", msg);
                        },

                        _ => {
                            debug!("unimplemented event received in server event loop");
                        },
                    };
                }
            }
        }
        Ok(())
    }
}

pub struct Server {
    config: ServerConfig,
    endpoints: Vec<Arc<EndpointContact>>,
    event_sink: mpsc::Sender<ServerEvent>,
    event_source: mpsc::Receiver<ServerEvent>,
}












