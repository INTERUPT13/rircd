use std::sync::Arc;
use color_eyre::{eyre::eyre, Result};
use futures::StreamExt;
use crate::endpoint::{Endpoint, EndpointContact};
use futures::stream::FuturesUnordered;

use log::{info,debug,trace};

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
        Server {
            config: ServerConfig::default(),
            endpoints: Vec::new(),
        }
    }

    async fn try_start_all_endpoints(&self, endpoints_to_start: Vec<Endpoint>) -> Vec<Result<()>>  {
        // TODO is there a better way?
        let results: FuturesUnordered<_> = endpoints_to_start.into_iter()
            .map(|ep| ep.start()).collect();
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
        }
        Ok(())
    }
}

pub struct Server {
    config: ServerConfig,
    endpoints: Vec<Arc<EndpointContact>>,
}
