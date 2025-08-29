pub fn login(cred: &models::Credentials) {
    super::database::get_user(cred);
}

pub mod models;