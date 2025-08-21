fn main() {
    /* 
    let a: i8 = 127; // Type explicitly giving
    let b = "adi"; // Type inference 

    let c = 23i32;
    let d = 23_i32; // underscores gets ignored
    let e = 1_00_000; // Just like commas

    let f = 0b01;
    let g = 0o77;
    let h = 0xff;
    */

    // let a: u8 = random_something() + 100;
    // let b: u8 = random_something().checked_add(100).expect("Error");
    let (a, _err) = random_something().overflowing_add(100);

    // println!("{a}"); // Panics in normal; release in 44
    // println!("{b}"); 
    println!("{a}");
    
    // Tuple
    let me = ("Aditya", 22, true, 1_00_000);
    let (x, y, z, a) = me;
    println!("{x} {y} {z} {a}");

    // Arrays and vectors
    let months: [&'static str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    let k: [i32; 5] = [10; 5];

}

fn random_something() -> u8 {
    200
}