/*
* Rust is an expression based language
* -- Most of the things are evaluated and return some value
* Expression values coalesce to a single point
* -- Can be used for nesting logic; e.g. if else, match
* -- Best not ot use more than a few levels of nesting
*/

// fn main() {
//     let my_num: i32 = 10;

//     let is_lt_5 = if my_num < 5 { true } else { false };
//     println!("{}", is_lt_5);

//     let is_lt_5 = my_num < 5;
//     println!("{}", is_lt_5);

//     let message = match my_num {
//         1 => "one",
//         2 => "two",
//         3 => "three",
//         _ => "many",
//     };
//     println!("{}", message);
// }

enum Menu {
    Burger,
    Fries,
    Drink,
}

fn main() {
    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";

    let order_placed: bool = match item {
        Menu::Drink => {
            if drink_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
        // This is incredibly confusing
    };

    println!("{}", order_placed);
}
