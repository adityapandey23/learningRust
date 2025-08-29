use std::collections::HashMap;

fn main() {
    let mut score: HashMap<String, i32> = HashMap::new();

    let yellow_team = String::from("Yello Team");
    let blue_team = String::from("Blue Team");
    let green_team = String::from("Green Team");
    let red_team = String::from("Red Team");

    score.insert(yellow_team, 10);
    score.insert(blue_team, 20);
    score.insert(green_team, 30);
    score.insert(red_team, 40);

    println!("{:?}", score);

    println!("Score of blue_team is {}", score.get(&String::from("Blue Team")).copied().unwrap_or(0)); // Retuns Optional<&V>

    score.insert(String::from("Blue Team"), 5); // Overriding
    
    score.entry(String::from("Blue Team")).or_insert(50); // Add if doesn't exist already

    // Looping over HashMap
    for (key, val) in &score {
        println!("{:?} => {:?}", key, val);
    }
    

    for (key, val) in &score {
        println!("{:?} => {:?}", key, val);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, val) in &map {
        println!("{:?} => {:?}", key, val);
    }
}
