// User related Code
/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(username: String, email:String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0
    }
}

fn print_user(user: &User) {
    println!("Username is {}", user.username);
    println!("email is {}", user.email);
    println!("active is {}", user.active);
    println!("sign_in_count is {}", user.sign_in_count);
}
*/

// Shapes and color related code
/*
struct Color(u8, u8, u8);
enum ShapeType {
    Circle,
    Rectangle
}
struct Shape{
    shape: ShapeType,
    color: Color
}

fn build_shape(shape_type: ShapeType) -> Shape {
    let shape = Shape{
        shape: shape_type,
        color: Color(0, 0, 0)
    };
    shape
}

fn change_color(shape: &mut Shape, delta: Color) {
    shape.color.0 += delta.0;
    shape.color.1 += delta.1;
    shape.color.2 += delta.2;
}

fn print_shape(shape: Shape) {
    match shape.shape {
        ShapeType::Circle => println!("The shape is a Circle"),
        ShapeType::Rectangle => println!("The shape is a Rectangle")
    }
    println!(
        "It's color in RGB values are R = {} G = {} B = {}",
        shape.color.0, shape.color.1, shape.color.2
    )
}
*/

// Hero related code
struct Point(u8, u8, u8);
struct Hero {
    name: String,
    position: Point
}

fn build_hero(name: String) -> Hero {
    let hero = Hero {
        name,
        position: Point(0, 0, 0)
    };
    hero
}

fn move_hero_1(mut hero: Hero, delta: Point ) -> Hero {
    hero.position.0 += delta.0;
    hero.position.1 += delta.1;
    hero.position.2 += delta.2;
    hero
} // Calling as : hero = move_hero_1(hero, dx, dy, dz);

fn move_hero_2(hero: &mut Hero, delta: Point) {
    hero.position.0 += delta.0;
    hero.position.1 += delta.1;
    hero.position.2 += delta.2;
} // Calling as : move_hero_2(&mut hero, dx, dy, dz);

fn move_hero_3(hero: &mut Hero, delta: Point) -> &mut Hero {
    hero.position.0 += delta.0;
    hero.position.1 += delta.1;
    hero.position.2 += delta.2;
    hero
} // Calling as : move_hero_3(&mut hero, dx, dy, dz).y_pos += 5; // Chaining via return

fn print_hero(hero: &Hero) {
    println!("Hero name is {}", hero.name);
    println!(
        "And it's position is x = {} y = {} z = {}",
        hero.position.0, hero.position.1, hero.position.2
    )
}

fn main() {
    // User related code
    /*
    let mut user_1 = User {
        email: String::from("something@gmail.com"),
        active: true,
        sign_in_count: 0,
        username: String::from("something@123")
    };

    user_1.username = String::from("somethingsomethingelse");
    // let s = user_1.username;
    // We cannot do this because, it moves the value from the user_1 to s and then we can't use it
    // let s = user_1.active; // This throws no error as it doesn't move
    
    // This works as we gave it the new value
    let _s = user_1.username;
    user_1.username = String::from("I am the new owner");

    let user_2 = User {
        email: String::from("something2@gmail.com"),
        active: true,
        sign_in_count: 0,
        username: String::from("something2@123")
    };

    let user_3 = build_user(String::from("Haha"), String::from("hehe"));

    // // Making a struct using other value of struct
    // let user_4 = User {
    //     email: String::from("lalal"),
    //     ..user_1 // But the issue is that the values are moved
    // };


    print_user(&user_1);
    println!("\n\n");
    print_user(&user_2);
    println!("\n\n");
    print_user(&user_3);
    println!("\n\n");
    // print_user(&user_4);
    */

    // Shape related code
    /*
    let mut shape_1 = build_shape(ShapeType::Rectangle);
    let mut shape_2 = build_shape(ShapeType::Circle);

    change_color(&mut shape_1, Color(2, 3, 4));
    change_color(&mut shape_2, Color(4, 3, 2));

    print_shape(shape_1);
    print_shape(shape_2);
    */


    // Hero related code
    let mut hero_1 = build_hero(String::from("Aditya"));
    let mut hero_2 = build_hero(String::from("Also Aditya"));
    let mut hero_3 = build_hero(String::from("Another Aditya"));

    println!("Before");
    print_hero(&hero_1);
    print_hero(&hero_2);
    print_hero(&hero_3);


    hero_1 = move_hero_1(hero_1, Point(1, 2, 3));
    move_hero_2(&mut hero_2, Point(1, 2, 3));
    move_hero_3(&mut hero_3, Point(1, 2, 3)).position.2 += 3; // Chaining


    println!("\n\n\nAfter");
    print_hero(&hero_1);
    print_hero(&hero_2);
    print_hero(&hero_3);
}