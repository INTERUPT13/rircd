use tokio::sync::oneshot;

#[derive(Debug)]
pub enum EventType {
    ShutdownServer(String),
}

#[derive(Debug)]
pub enum EventResponse {
}

// type for events passed to endpoints. 
// .event field -> actual event
// .response_sink -> acting on an event always produces a response.
//                  it is meant to be send back here


// TODO impl Display for event ourself (derived one is ugly) so
// we can then use it in the logs n shit
#[derive(Debug)]
pub struct Event {
    pub event: EventType,
    pub response_sink: oneshot::Sender<EventResponse>
}
