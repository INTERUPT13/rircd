use color_eyre::Result;
use crate::endpoint::EndpointBackend;
use async_trait::async_trait;
use log::{info};

pub struct IrcEndpoint {
}

impl Default for IrcEndpoint {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl EndpointBackend for IrcEndpoint {
    async fn start(self: Box<Self>, mut name: String) -> Result<()> {
        info!("starting IRC endpoint {}", name);
        Ok(())
    }
}


