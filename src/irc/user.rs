



pub struct IrcUser {
    username: String,
    realname: String,
}

impl IrcUser {
    pub fn from_username_and_realname(username: &String, realname: &String) -> Self {
        Self {
            realname: realname.clone(),
            username: username.clone(),
        }
    }
    pub fn from_username(username: &String) -> Self {
        let username = username.clone();
        Self {
            realname: username.clone(),
            username,
        }
    }
}
