/* 
fn main() {
    let s = "Hello World";

    {
        let x = "Hey from x";
    }

    println!("{s}");
    // println!("{x}"); // Out of scope
}
*/

/* 
fn main() {
    let mut s1 = "Hello World";
    s1 = "Hellow World!";
    println!("{s1}");

    let mut s2 = String::from("Hello world"); // Heap, growable
    s2.push_str("!");
    println!("{s2}");
}
*/

/* 
fn main() {
    let mut x = 5;
    let y = x; // Copy
    
    x = 10;

    println!("x is {x} and y is {y}");
}
*/

/* 
fn main() {
    let s1 = String::from("I am x");
    let s2 = s1;

    // Only one owner of the memory 
    // println!("This is s1: {s1} and this is s2: {s2}"); // This erros
    // Shallow copy kinda but here it's called a move1
    println!("This is s1: and this is s2: {s2}");
}
*/

/* 
fn main() {
    let mut s1 = String::from("Aditya");
    let s2 = s1.clone(); // forces out to make a copy on the heap
    // Here it's expensive by it's a you problem not a me problem

    s1.push_str(" Pandey");

    println!("This is s1: {s1} and this is s2: {s2}"); 

    // Copy and Drop trait
}
*/

/* 
fn main() {
    let x = 10; 
    let y = 15;

    let sum = add(x, y); // Because it uses, Copy Trait
    println!("Sum of x = {x} and y = {y} is sum = {sum}");
    
    let name = String::from("Aditya Pandey");
    takes_ownership(name);
    println!("Value of name is {name}"); // Here it doesn't work because the owner is now takes_ownership() function not main function
    
    /* 
    // Here it works as first, this main function has the ownership
    // Later, it's taken by the takes_ownership() function
    println!("Value of name is {name}");
    takes_ownership(name);
    */

}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn takes_ownership(s : String) {
    println!("Inside ownership: {s}");
}
*/

/* 
fn main() {
    let a = gives_ownership();
    let b = a;

    println!("a = {a} and b = {b}");
}

fn gives_ownership() -> String{
    let s = String::from("Aditya");
    s
}
*/

/* 
fn main() {
    let mut name = String::from("Aditya"); // We did mut here because we wanted to have back the ownership
    name = takes_and_gives_back(name);
    println!("We can still use {name}");
}

fn takes_and_gives_back(s: String) -> String {
    println!("Hey there {s}. How are you doing?");
    s
}
*/

fn main() {
    let name = String::from("Aditya Pandey");
    let (name, le)= cal_len(name); // We  don't need mut over here

    println!("The length of {} is {}", name, le);

}

fn cal_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}