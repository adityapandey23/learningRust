/*
 * Return the first occurence of space in a string, if none is found, return the length it
*/ 


/*
fn get_space_mine(s: &String) -> usize {
    let mut cnt = 0;
    for ch in s.chars() { // .chars() makes the string iterable
        if ch == ' ' {
            break;
        }
        cnt += 1;
    }
    cnt
}
*/

/*
fn main() {
    // let s1 = String::from("AdityaPandey");
    // let n1 = get_space_mine(&s1);
    // println!("For string {} space occurs at {}", s1, n1);
    
    let mut s2 = String::from("AdityaPandey");
    let n2 = get_space_docs(&s2);
    s2.clear(); // Okay, so the problem is we have altered the data, but the value of it is valid
                // and can lead to potential problems
    println!("For string {} space occurs at {}", s2, n2);
}

fn get_space_docs(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
*/

// Don't look at my code stranger :p

fn main() {
    let s2 = String::from("AdityaPandey");
    let size = get_space_docs(&s2);
    // println!("Size of {} is {}", s2, size.len()); // Here it'll work as the refernce goes out of
    // scope so is destroyed
    // s2.clear(); // Mutation here

    println!("Size of {} is {}", size, size.len()); // Here it won't work as the while in the scope
                                                  // of refernce we tried to alter the values so it
                                                  // erros out 4 lines above

}

// here in the function def, we can use both, &String and &str, just because it'll let us this
// function more widely used

// Fun fact: the reason why string literals are &str because, it creates a single space for the
// actual string, and then can let point multiple variables
//
// let a = "something";
// let b = "something";
// Here there is only one "something" in the memory, by two variables points to it
// This is an optimisation done by compiler as the values are known at the compile time
// Aka String interning

fn get_space_docs(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
