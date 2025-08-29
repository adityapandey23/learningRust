// Plain 
/*
fn main() {
    let w = 100; 
    let h = 100;
    let area = calculate_area(w, h);
    println!("Area = {}", area);
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}
*/

// Using tuples
/*
fn main() {
    let dimension = (100, 100);
    let area = calculate_area(dimension);
    println!("Area = {}", area);
}

fn calculate_area(dimension : (u32, u32)) -> u32 {
    let (w, h) = dimension;
    w * h
}
*/

// Using Struct
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = Rectangle {
        height: 100,
        width: 100
    };

    let area = calculate_area(&rect);

    // println!(
    //     "Area = {}, when height = {} and width = {}",
    //     area, rect.height, rect.width
    // );

    println!("Area = {} where rectangle is {:?}", area, rect);

    // dbg!(rect); // Takes ownership
    // dbg!(&rect); // Doesn't takes ownership
    // We can do like this as well let area = dbg!(calculate_area(...))
}
