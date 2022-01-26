


#[derive(Debug)]
pub enum IrcEventType {
    Nick(String)
}

#[derive(Debug)]
pub struct IrcEvent {
    pub event: IrcEventType,
}
