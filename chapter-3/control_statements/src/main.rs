fn main() {
    // If, else if and else statements 
    if 5 < 10 {
        println!("Woho");
    }
    else if 4 > 2 {
        println!("nananan");
    }
    else {
        println!("Nah");
    }
    
    if is_even(23) {
        println!("Even");
    }
    else {
        println!("Odd");
    }

    // Technically 1 isn't true over here and 0 isn't false

    let ap = if 23 % 2 != 0 {true} else {false}; // Value type should be same
    println!("what is ap {ap}");

    // Loop
    let mut cnt = 1;
    loop {
        if cnt == 60 {
            println!("We are freee");
            break;
        }
        else {
            println!("You are under my control wuhahah");
        }
        cnt += 1;
    }

    // while
    while cnt >= 0 {
        println!("Why are we runnin?");
        cnt -= 1;
    }

    // for
    let days = ["mon", "tue", "wed", "thus", "fri", "sat", "sun"];
    // This is more efficient as it doesn't have to check for whether you are in bounds or not

    for day in days {
        print!("{day} ");
    }

    let mut num = 1;
    let result = loop {
        println!("Value of number is {num}");

        if num == 64 {
            break 70;
        }
        num += num;
    };

    println!("In the end: {result}");

    let mut i = 0; let mut y;
    // Using labels for loops
    'outer_loop: loop {
        if i == 5 {
            break 'outer_loop;
        }

        y = 0;
        'inner_loop: loop {
            if y > i {
                break 'inner_loop;
            }
            print!("{y} ");
            y += 1;
        }
        println!();

        i += 1;
    }

    for x in 1..=10{ // We can do rev as well
        println!("x is {x}");
    }
}

fn is_even(x: i32) -> bool {
    if x & 1 == 1 {
        return false;
    }
    return true;
}