/*

#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user(cred: &super::auth_utils::models::Credentials) {
    }
    // We can nest modules
}

pub mod auth_utils {

    pub fn login(cred: &models::Credentials) {
        // crate::database::get_user(cred); // Way - 1
        super::database::get_user(cred); // Way - 2
    }

    pub mod models {

        pub struct Credentials {
            pub username: String,
            pub password: String
        }
    }
}


pub fn authenticate(cred: &auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {

    }
}

pub mod utils;
// src/utils.rs (preferred)
// src/utils/mod.rs

*/
#![allow(dead_code, unused_variables)]
use database::{Status, connect_to_database};
use auth_utils::login;

pub fn authenticate(cred: &auth_utils::models::Credentials) {
    if let Status::Connected = connect_to_database() {
        login(&cred);
    }
}

pub mod utils;
pub mod database;
pub mod auth_utils;