/*
// This way we'll violate the DRY princinples
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for element in list {
        if *element > largest {
            largest = *element;
        }
    }
    largest
}
*/

/*
// Trait bound -> Limiting the traits
fn largest_from_list<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for element in list {
        if element > largest {
            largest = element;
        }
    }
    largest
}

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6];
    let l = largest_from_list(&list);
    println!("Largest number in the list is {l}");
}
*/

// We need ordering as well as display trait bound over here
struct Point <T: std::cmp::PartialOrd + std::fmt::Display> { // This is how we add multiple generic
                                                             // bound
    // We can also add multiple generics i.e. T and U
    x: T,
    y: T
}

impl <T: std::cmp::PartialOrd + std::fmt::Display> Point<T> {
    fn print(&self) {
        println!("The value of x is {} and the value of y is {}", self.x, self.y);
    }
}

// Woala, Polymorphism
impl Point<i32> {
    fn cal(&self) -> i32 {
        4
    }
}

impl Point<f64> {
    fn cal(&self) -> f64 {
        4.0
    }
}


fn main() {
    let point_a = Point {
        x: 2,
        y: 3
    };

    point_a.print();
    point_a.cal();


    let point_b = Point {
        x: 2.2,
        y: 3.3
    };

    point_b.print();
    point_b.cal();

    // Fast as monomorphization -> compile time realise and create options
}
