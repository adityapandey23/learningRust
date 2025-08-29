// Lints vs Traits 
// #[warn(dead_code)] // This is a lint

/*
// Basic Usage
enum Coin {
Penny,
Nickel,
Dime,
Quarter
}

fn value_in_cents(coin: &Coin) -> u8 {
// if only take boolean here we can give types as well
match coin {
Coin::Penny => 1, // This is called an arm
Coin::Nickel => 5,
Coin::Dime => 10,
Coin::Quarter => {
// For multiple lines code we can use this curly braces
25
}
}
}
*/

/*
#[derive(Debug)]
enum UsState {
Alabama,
Alaska
}

enum Coin {
Penny,
Nickel,
Dime,
Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u8{
match coin {
Coin::Penny => 1, 
Coin::Nickel => 5,
Coin::Dime => 10,
// // Way 1
// Coin::Quarter(UsState::Alabama) => {
//     println!("It's from Alabama");
//     25
// }
// Coin::Quarter(UsState::Alaska) => {
//     println!("It's from Alaska");
//     25
// }
// Way 2
Coin::Quarter(state)  => { // Make sure to keep this last otherwise there will be a dead
                           // code condition for the above way as the pointer will never
                           // reach here due to the always matching condition
                           println!("Got Quarter value of {:?}", state);
                           25
                           }
                           }

}


fn main() {
let coin1 = Coin::Penny;
println!("Value is {}", value_in_cents(&coin1));

let coin2 = Coin::Quarter(UsState::Alabama);
println!("Value is {}", value_in_cents(&coin2));
}
*/

/*
// Matching with values of Option<T>
fn add(num1: i32, num2: &Option<i32>) -> i32 {
    match num2 {
        Some(i) => num1 + i,
        None => num1
    } // Matches are exhaustive though
}


fn main() {
    println!("Added Result of {} and {:?} is {:?}",
        1, Option::Some(1), add(1, &Option::Some(1))
    );


    println!("Added Result of {} and {:?} is {:?}",
        1, "None", add(1, &Option::None)
    );
}
*/

// Catch all pattern
fn main() {
    let dice_roll = 3;
    match dice_roll {
        3 => println!("Your got a fancy hat"),
        6 => println!("Your fancy hat was taken away"),
        other => println!("Move {} steps", other)
        // _ => reroll() // if we don't want to use that value
        // _ => () // Doing nothing lol
    };
}
