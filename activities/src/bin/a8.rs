// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum Flavors {
    Cola,
    Grape,
    Lime,
}

struct Drink {
    flavor: Flavors,
    ounces: f64,
}

fn drink_info(drink: Drink) {
    println!(
        "This is {:?} flavor. It is {:?} oz",
        drink.flavor, drink.ounces
    )
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavors::Cola => drink_info(drink),
        Flavors::Grape => drink_info(drink),
        Flavors::Lime => drink_info(drink),
    }
}
fn main() {
    print_drink(Drink {
        flavor: Flavors::Cola,
        ounces: 12.00,
    });
    print_drink(Drink {
        flavor: Flavors::Grape,
        ounces: 12.00,
    });
    print_drink(Drink {
        flavor: Flavors::Lime,
        ounces: 12.00,
    });
}
