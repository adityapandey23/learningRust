/*
// Basic without any struct
#[derive(Debug)]
enum IpAddrKind {
V4,
_V6
}

fn main() {
route("1.2.3.4", IpAddrKind::V4)
}

fn route(ip: &str, kind: IpAddrKind) {
println!("Routing request to IP {} of kind {:?}", ip, kind);
}
*/

/*
// Using struct to make it more robust 
use regex::Regex;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

struct IpAddress {
    address: String,
    kind: IpAddrKind
}

impl IpAddress {
    fn new(address: &str) -> Self {

        let ipv4_pattern = Regex::new(r"[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}").unwrap();
        let ipv6_pattern = Regex::new(r"[0-9A-F]{1,4}\:[0-9A-F]{1,4}\:[0-9A-F]{1,4}\:[0-9A-F]{1,4}\:[0-9A-F]{1,4}\:[0-9A-F]{1,4}\:[0-9A-F]{1,4}\:[0-9A-F]{1,4}").unwrap();

        if ipv4_pattern.is_match(address) {
            IpAddress {
                address: address.to_string(),
                kind: IpAddrKind::V4
            }
        }

        else if ipv6_pattern.is_match(address) {
            IpAddress {
                address: String::from(address),
                kind: IpAddrKind::V6
            }
        }

        else {
            panic!("Wrong IP type, why are you like this ?");
        }
    }
}

fn main() {
    let mine_ip = IpAddress::new("127.0.0.0");
    route(&mine_ip);


    let your_ip = IpAddress::new("ABCD:EF01:2345:6789:ABCD:EF01:2345:6789");
    route(&your_ip);


    // Just chill
    // let someone_ip = IpAddress::new("baigan");
    // route(&someone_ip);
}

fn route(ip_address: &IpAddress) {
    println!("Routing request to IP {} of kind {:?}", ip_address.address, ip_address.kind);
}
*/

/*
// Only using enum
#[derive(Debug)]
enum IpAddrKind {
    V4(String), // could do like (u8, u8, u8, u8)
    V6(String)
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    route(home);


    let office = IpAddrKind::V6(String::from("127.0.0.1"));
    route(office);
}

fn route(ip: IpAddrKind) {
    println!("Routing request to {:?}", ip);
}
*/

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

/*
 * Equivalent to 
 * struct QuitMessage;
 * struct MoveMessage {
 *      x: i32,
 *      y: i32
 * }
 * struct WriteMessage(String);
 * struct ChangeColorMessage(i32, i32, i32);
 */

// We can define functions on enums as well
impl Message {
    fn _call(&self) {
        // Do something
    }
}

fn main() {
    let _a = Some(1); // This is also an enum
                                // Also, it's already there in the prelude
    // Well it's basically value can be something or nothing
    // RUST DOESN'T HAVE THE NULL TYPE ? WTF ?
    

    let _b = 2;
    // let c = a + b; // We cannot perform this operations between an Option and a normal int value


    let _c : Option<i32> = None; // This is how we define an optional value
    // Different operations are there in this
    // unwrap and unwrap_or()

}
