#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Implementaion block
impl Rectangle {
    fn calculate_area(self: &Self) -> u32 { // self is kinda like this in java or cpp
        self.height * self.width
    }

    /*
    // Basically we can have setter and getters kinda thing here
    fn width(&self) -> u32 { // Same name as field is possible here
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
    */

    fn can_cover(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(side: u32) -> Self {
        Rectangle { width: side, height: side } 
    }

    // Does automatic referencing and de-referencing for pointer, Rust ftw
}

fn main() {
    let rect1 = Rectangle{
        height: 100,
        width: 100
    };

    println!(
        "The area of the {:?} is {}",
        rect1, rect1.calculate_area() // Method syntax 
    );


    let rect2 = Rectangle{
        height: 50,
        width: 50
    };

    println!(
        "The area of the {:?} is {}",
        rect2, rect2.calculate_area() // Method syntax 
    );

    if rect1.can_cover(&rect2) {
        println!("{:?} can cover {:?}", rect1, rect2)
    }

    if rect2.can_cover(&rect1) {
        println!("{:?} can cover {:?}", rect2, rect1)
    }
    
    let sq = Rectangle::square(5); 
    // Kinda like a static function in other languages (Something related to this and namespace
    // will com in chapter - 7)
    println!("The area of {:?} is {}", sq, sq.calculate_area());
}
