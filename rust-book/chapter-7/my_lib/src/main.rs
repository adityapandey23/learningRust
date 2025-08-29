mod garden;
use crate::garden::vegetables::Asparagus;

fn main() {
    // For type A enum
    let something: Asparagus;
    something = Asparagus::TypeA;
    match something {
        Asparagus::TypeA => println!("Asparagus Type A"),
        Asparagus::TypeB => println!("Asparagus Type B"),
    }

    // For type B enum
    let something_else: Asparagus;
    something_else = Asparagus::TypeB;
    match something_else {
        Asparagus::TypeA => println!("Asparagus Type A"),
        Asparagus::TypeB => println!("Asparagus Type B"),
    }
}
