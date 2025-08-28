/*
fn main() {
    let result = match divide(4, 0) {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {err}");
            -1
        }
    };
    println!("R = {result}");
}

fn divide(x: i32, y: i32) -> Result<i32, String> { // In prelude
    if y == 0 {
        return Err(String::from("Please do not divide by zero"));
    }
    Ok(x / y)
}
*/

/*
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem in creating file {e}")
            },
            _ => panic!("Something went wrong {error}")
        },
    }; // Will do when I do closure functions 

    println!("{:?}", greeting_file);
}
*/

// Instead of checking at each step like golang we can use a question mark

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // The ? performs an early return with error
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    println!("{:?}", read_username_from_file());
}
