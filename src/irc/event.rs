

pub struct IrcEvent {
}

//since it might also be desireable to be able to communicate with the
//acceptor threads as well this type was introduced
pub struct IrcAcceptorEventIn {
}
pub struct IrcAcceptorEventOut {
}

// used to represent the irc messages the IrcConnecton*::handler() either
// parses in or spits out
pub struct IrcConnectorEventOut {
}

pub struct IrcConnectorEventIn {
}
