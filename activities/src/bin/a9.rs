// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_coords(x: f64, y: f64) -> (f64, f64) {
    (x, y)
}

fn y_and_five(y: f64) -> &'static str {
    if y > 5.0 {
        "y is greater than 5"
    } else if y < 5.0 {
        "y is less than 5"
    } else {
        "y is equal to 5"
    }
}

fn main() {
    let (x, y) = get_coords(3.65625650, 5.1616165);
    println!("The x-coord is {}", x);
    println!("The y-coord is {} and  {}", y, y_and_five(y));
}
