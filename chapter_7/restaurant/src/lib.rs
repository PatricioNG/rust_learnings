mod front_of_house;

pub use crate::front_of_house::hosting;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //Super refers to the parent module
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        // A field is private by default within a public struct
        // unless explicitly declared
        seasonal_fruit: String,
    }

    //Public enums - all fields are public when using the pub keyword
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    //`use` can be used to bring the crate path into scope
    // use crate::front_of_house::hosting;

    // For functions, bring the parent module into scope
    // For structs or enums, bring the full path into scope ie:
    // use std::collections::HashMap;

    //As exists to solve name collisions
    // use std::fmt::Result;
    // use std::io::Result as IoResult;

    // fn function1() -> Result {
    //     // --snip--
    // }

    // fn function2() -> IoResult<()> {
    //     // --snip--
    // }

    //Absolute path - This is the preferred method
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
