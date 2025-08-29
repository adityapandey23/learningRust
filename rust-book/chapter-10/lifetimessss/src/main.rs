/*
// Issue
fn main() {
    let r: &i32;

    {
        let a = 10;
        r = &a;
    }

    println!("{}", r);
}
*/

/*
fn main() {
    let s1 = String::from("Aditya");
    let s2 = String::from("Aditya Pandey");

    let result = longest(&s1, &s2);

    println!("{}", result);
}

// ' denote a lifetime
// The life time of the result will be as long as
// The reference of x and y is valid -> Technically of the smaller lifetime one
// As the shorter lifetime is of s2
// This will get the lifetime of s2 instead of s1
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
*/

/*
// Lifetimes in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn printing(s: String) {
    println!("{}", s);
}

fn main() {
    let novel = String::from("Call me Aditya. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence
    };

    // printing(novel); 
    // Novel lifetime ended here so i's lifetime ended here as well that's why we cannot use it in
    // the next line
    
    println!("{}", i.part);
}
*/


/*
fn main() {
    // one parameter -> one life time
    // two parameter -> two life times
    // if one lifetime parameter and other not -> that lifetime is assigned to each
    //
}
*/


// Static lifetime
fn main() {
    let s1 = "Aditya"; 
    // All the string literals are 'static
    // It's directly embedded into the binary
    let result: &str;

    {
        let s2 = "Aditya Pandey";
        result = longest(s1, s2);
    }

    println!("Longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
