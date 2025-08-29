fn main() {
    println!("From the main function");
    another_fn();
    let f = fact(5);
    println!("Factorial of 5 is {f}");

    // Statement -> Do something but don't return a value 
    // Can't do x = y = 8 over here in Rust
    // Expression -> Evalulate to a value

    let g = add_two_num(23, 25);
    println!("Sum of 23 and 25 is {g}");
}

fn another_fn() {
    println!("From another function");
}

fn fact(i: u32) -> u32 {
    if i <= 1 {
        return 1;
    }
    i * fact(i-1)
}

// Function definition is a statement but function callings is not a statement
fn add_two_num(a : i32, b: i32) -> i32 {
    a + b
    // This is an expression and this makes the whole block evalulate to a value
    // Semicolon matters here as without semicolon the line is a expressions
    // Other wise it becomes a statement

    // return a + b; // Statement

}
