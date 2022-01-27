// structure representing events that are send from an endpoint to the server
// these can be:
// -instructions to the server state:  config changes / shutdown / add/rm endpoint
// -request to route messages to Users/Channels eg:
//
//                                                                       |----> endpoint2
//                                                                       |
//      endpoint1 ---(ServerEvent)---> server --(EndpointBackendEvent) --|----> endpoint3
//                                                                       |
//                                                                       |----> endpoint4
pub struct ServerEvent {
}

// structure representing events that are send from the server to an endpoint
pub struct EndpointBackendEvent {
}

// the EndpointBackend implementation specific Events types are to be found
// in src/<BACKEND>/event.rs where <backend> could be "irc"

