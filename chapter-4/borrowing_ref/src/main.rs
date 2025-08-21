/*
 * So, if we want to modify a reference we have to do the following things
 *  1. Change the data types of the function parameter to mutable ref like this &mut String
 *  2. Change the ref to mut ref where you are calling the function like &name to &mut name
 *  3. Then finally make the original variable mutable obviously
 */

/*
fn main() {
    let mut name = String::from("Aditya Pandey");
    let size = cal_len(&mut name); 

    println!("The size of {} is {}", name, size);
}

fn cal_len(s: &mut String) -> usize { // This is what we call borrowing
    s.push_str(" is a very good boy");
    s.len()
}
*/

/*
 * At a time, only one variable can have a mutable reference of another variable
 */

/*
fn main() {
    let name = String::from("Aditya Pandey");
    // Normal reference works
    let s1 = &name;
    let s2 = &name;

    // This won't work
    // let s1 = &mut name;
    // let s2 = &mut name;

    // This won't work either, no type of reference can be takes if one has take a mutable reference
    // let s1 = &mut name;
    // let s2 = &name; // Neither function calling will work
    // Because you can't borrow an immutable ref when you have borrowed mutable ref and vice versa
    // is also true | Here the exception being, you never used those so it get's created and
    // destroyed in the name line, just look at the next to next example and you'll understand


    println!("s1 = {} and s2 = {}", s1, s2);
}
*/


// Makes sense, this will prevent race conditions without the needing of locks mechanisms....noice
/*
fn main() {
    let mut name = String::from("Aditya Pandey");

    {
        let sentence = &mut name;
        sentence.push_str(" is a very good boy learning Rust");
        println!("{}", sentence); // Mutable ref goes out of scope so yeah, it get's destroyed
    }
    
    let sentence = &mut name; // This is the reason why here we can have a mutable reference
    sentence.push_str("is saying hello");
    println!("{}", name);

}
*/


// It seems that the immutate reference won't let us create any mutable reference up until it's
// scope is valid, once it goes out of scope, it'll let us do whatever we want
/*
fn main() {
    let mut name = String::from("Aditya Pandey");
    let s1 = &name; // Get's created and destroyed at the same time
    let s2 = &name; // Get's created and destroyed at the same time
    let length = cal_len(&name); // Same scenario is here
    let s3 = &mut name;

    println!("{} has a length of {}", s3, length);
}

fn cal_len(s: &String) -> usize {
    s.len()
}
*/


// Dangling reference
// Rust says, hold up, I'll take care of this
/*
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::new();
    &s // Here the owner of s is dangle, it ends, s is released, but, it returns a ref to main, now
       // that ref is referring to no where, as originally where it was referring, has been cleaned
       // up
}
*/

// So instead, just return it's ownership i.e move the ownership
fn not_dangle() -> String {
    let mut s = String::from("Aditya Pandey");
    s.push_str(" is a very good boy"); // We can return the ownership of a mutable object as an
                                       // immutable object
    s
}

fn main() {
    println!("Are you seriously looking at my code ?");
    let name = not_dangle();
    println!("{}", name);
}













