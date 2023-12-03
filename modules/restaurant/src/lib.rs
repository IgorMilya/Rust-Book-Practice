pub use auth_utils::modules::Credentials;
use database::Status;

mod database;
mod auth_utils;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}