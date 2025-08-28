
pub enum Status {
    Connected,
    Interrupted
}

pub fn connect_to_database() -> Status {
    Status::Connected
}

pub fn get_user(cred: &super::auth_utils::models::Credentials) {
    // Do something to get the user
}