/*
fn main() {
    /*
    let mut vec : Vec<i32> = Vec::new(); // Pushing to init vec is an anti pattern
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    let vec = vec; // Shadowing to make it immutable
    */

    // Here this is a macro
    let mut vec = vec![1, 2, 3, 4]; // If we don't mutate it then it's final value
    vec.push(5);

    println!("{:?}", vec);


    // Two ways to access the elements of a Vec
    let first_value = &vec[0];
    println!("First value = {}", first_value);

    // let fourth_value = vec.get(3).unwrap_or(&-1); // One way to do it 
    let fourth_value = match vec.get(3) {
        Some(value) => value,
        None => {
            println!("Index out of bounds");
            &-1
        }
    };
    println!("Fourth value = {}", fourth_value);
}
*/

/*
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let first = &vec[0];

    // vec.push(1); // Won't let us do, because, first is an immutable reference
                 // Sometimes adding or subtracting might change memory locations and all
                 // so the reference might do invalue

    println!("First element is {first}");
}
*/

/*
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        // print!("{}", i); // Printing works without deref 
        // print!("{}", *i * 2); // Here we do have to de-reference
        *i *= 2;
    }

    println!("{:?}", v);
}
*/

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let cells : Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Text(String::from("Aditya")),
        SpreadSheetCell::Int(22),
        SpreadSheetCell::Float(4.999)
    ];

    // println!("{:?}", cells);

    for i in &cells {
        println!("{:?}", i);
    }

    match &cells.first() {
        Some(SpreadSheetCell::Int(value)) => println!("The value in int {}", value),
        Some(SpreadSheetCell::Float(value)) => println!("The value in float {}", value),
        Some(SpreadSheetCell::Text(value)) => println!("The value in text {}", value),
        None => println!("It's none lol")
    }
}
