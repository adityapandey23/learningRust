/*
fn main() {
    // let greeting = String::from("नमस्ते");
    let s1 = String::from("hola");
    let s2 = "is greeting in Spanish";
    let s3 = String::from(".");
    let greeting = s1 + s2 + &s3; // Here, s1 is moved but s3's reference is passed
    // When we do add, the first one is the moved and the second one is added to it
    // This works even if the second arg is a String, it coerces to a slice ref
    // let greeting = s1 + &s3; // Like this
    // println!("{s1} {s2} {s3} {greeting}");
}
*/


/*
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let sentence = s1 + "-" + &s2 + "-" + &s3;

    let sentence = format!("{s1}-{s2}-{s3}"); // It uses references
    println!("{sentence}");
}
*/

/*
fn main() {
    let hello1 = String::from("Hello");
    let hello2 = String::from("Здравствуйте");

    println!("{hello1} size is {}", hello1.len()); // Size -> 5
    println!("{hello2} size is {}", hello2.len()); // Size -> 24 (12 * 2 Bytes each)
    // So now if we access hello1[0] -> will give half byte for actual representation
}
*/

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Byte values
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    
    // Scaler values
    // ['न', 'म', 'स', '्', 'त', 'े']

    // Grapheme values
    // ["न", "म", "स्", "ते"]

    let hello = String::from("नमस्ते");

    for c in hello.bytes() {
        println!("c = {c}")
    }
    
    println!("------------------------------------------------------");

    for c in hello.chars() {
        println!("c = {c}")
    }
    
    println!("------------------------------------------------------");

    // For Grapheme we have to download another crate
    for c in hello.graphemes(true).collect::<Vec<&str>>() {
        println!("c = {c}")
    }

    println!("------------------------------------------------------");

} 
