use tokio::sync::oneshot;
use crate::message::Message;


#[derive(Debug)]
pub enum ServerEventType {
    ShutdownServer(String),
    ShutdownEndpoint(String),

    //UserConnect,
    //UserJoinChannel,
    //UserLeaveChannel,
    Message(Message),
}

#[derive(Debug)]
pub enum ServerEventResponse {
}

// type for events passed to endpoints. 
// .event field -> actual event
// .response_sink -> acting on an event always produces a response.
//                  it is meant to be send back here


// TODO impl Display for event ourself (derived one is ugly) so
// we can then use it in the logs n shit

// these events are meant to go to the server
#[derive(Debug)]
pub struct ServerEvent {
    pub event: ServerEventType,
    pub response_sink: oneshot::Sender<ServerEventResponse>
}

// events that are meant to be send to endpoints by the server
#[derive(Debug)]
pub struct EndpointEvent {
    pub event: EndpointEventType,
    pub response_sink: oneshot::Sender<EndpointEventResponse>
}

#[derive(Debug)]
pub enum EndpointEventType {
}

#[derive(Debug)]
pub enum EndpointEventResponse {
}

