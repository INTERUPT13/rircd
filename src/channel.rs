use tokio::sync::RwLock;
use std::sync::Arc;
use crate::endpoint::EndpointContact;


// represents channels in a more abstract fashion. Channels can be seen
// as some sort of grouping of multiple endpoints. Messages send to channels
// travel to all the endpoints within that channel except the one it was send from
pub struct Channel {
    motd: String,
    name: String,
    // since an Arc<Channel> is being passed around we need to rwlock this
    // if we want to be able to remove endpoints from channels
    endpoints: RwLock<Vec<Arc<EndpointContact>>>,
}
