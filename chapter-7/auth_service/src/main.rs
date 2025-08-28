use auth_service::auth_utils::models::Credentials;
use auth_service::authenticate;

fn main() {
    let cred = Credentials {
        username: String::from("Aditya"),
        password: String::from("mysecretpassword")
    };

    authenticate(&cred);
}
