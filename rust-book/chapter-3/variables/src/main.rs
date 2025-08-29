const PI: f64 = 3.14; // Data type is required over here; valid over the scope
// variables are always bind to a scope

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // This will work

fn main() {
    // Immutable variables are still not constants
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    // Constants
    
    println!("The value of PI is : {PI}");

    // Shadowing
    // Here we can change the value as well as datatype which we cannot in mut
    let x = x + 1; // Prev x shadowed by this
    {
        let x = true;
        println!("The value of x is : {x}");
    }

    println!("The value of x is : {x}");

    println!("Number of Seconds in three hours are {THREE_HOURS_IN_SECONDS}");

    // const THREE_HOURS_IN_SECONDS: u32 = age * 60 * 3; // This won't
    // Shadowing -> Mut to immut; type enforcing

}