//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     pub fn eat_at_restaurant() {
//         // Absolute path
//         crate::front_of_house::hosting::add_to_waitlist();
//
//         // Relative path
//         super::front_of_house::hosting::add_to_waitlist();
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }
mod front_of_house;

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String, //público
        seasonal_fruit: String, //privado
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub use crate::front_of_house::hosting;
//external code can now call the add_to_waitlist

pub fn eat_at_restaurant() {

    use self::front_of_house::hosting; // can call directly hosting:: / from relative self::
    use crate::front_of_house::hosting::add_to_waitlist; // can call directly add_to_waitlist() from root crate::

    hosting::add_to_waitlist();
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
     //meal.seasonal_fruit = String::from("blueberries");


    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}