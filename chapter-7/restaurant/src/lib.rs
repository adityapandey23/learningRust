mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    // Absolute -> crate
    // Relative -> self, super
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("Peaches")
            }
        }

    }
}

// If we follow the root level path, we have to change 
// many private items to publics as most of them are not
// directly accessible via the root crate
fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist(); // Because eat_at_restaurant are siblings i.e. on the same level

    // crate::front_of_house::hosting::add_to_waitlist(); // something like this


    let breakfast = back_of_house::Breakfast::summer("Wheat"); // As one field is private, we cannot directly create the instance of Breakfast

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Use works for the same scope not nested scope
    // Import till parent and then call the function on it
    // If you are using some sort of struct or enum then specify the whole path
    // We can do import as kinda shit here as well
    // TODO: Revise re-exporting
}

// structs and all have all the items private
// enums varients are by default public 