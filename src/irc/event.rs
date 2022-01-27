use std::net::SocketAddr;


#[derive(Debug)]
pub enum IrcEventType {
    Nick(String),
    // username, realname
    User(String,String),
}

#[derive(Debug)]
pub struct IrcEvent {
    pub event: IrcEventType,
    pub remote_addr: SocketAddr,
}
