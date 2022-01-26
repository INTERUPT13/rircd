use tokio::sync::oneshot;

pub enum EventType {
}

pub enum EventResponse {
}

// type for events passed to endpoints. 
// .event field -> actual event
// .response_sink -> acting on an event always produces a response.
//                  it is meant to be send back here
pub struct Event {
    event: EventType,
    response_sink: oneshot::Sender<EventResponse>
}
