#![allow(where_clauses_object_safety)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

#![feature(async_closure)]
#![feature(generators)]

mod server;
mod endpoint;
mod event;
mod channel;
// TODO make optional by cargo features
mod irc;

use crate::server::Server;
use crate::irc::IrcEndpoint;
use crate::endpoint::Endpoint;

use color_eyre::{eyre, Result};
//
//
// TODO when routing messages (default -> Msg goes to every endpoint except the sender) but it should

// 
// TODO I got the suspicion that mpsc.recv() will yield also if the channel  sender end is dropped
// which uhh well we gotta be aware of that or make sure it wont happen
// [the compiler shouldn't drop it if the code still uses it in some way later]
//
//TODO endpoint keepalive checks
//TODO event.send() timeout behavior (otherwise we risk hangs)
//

// TODO when spawning off endpoint's run() function. Why not make it a 2 stage process. Like
// actually we call a try_run() -> Result<()> function that does the shit that can fail like socket
// init with ? operators. And then when all the internal data structures are created it finally
// spawns off run() and returns Result<()>



#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let irc_ep =  Endpoint::new( "irc_1".into(), Box::new(IrcEndpoint::default()) );

    let mut s = Server::new();

    s.run({
        vec![ irc_ep ]
    }).await
}





