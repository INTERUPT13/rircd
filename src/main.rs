#![allow(where_clauses_object_safety)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_patterns)]

#![feature(async_closure)]
#![feature(generators)]

mod server;
mod endpoint;
mod event;
mod channel;
mod message;
// TODO make optional by cargo features
mod irc;

use color_eyre::{Result,eyre::eyre};


#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
