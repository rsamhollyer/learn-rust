/*
* A type of "record"
* Store data anonymously
* - No need to name fields
* Use to return pairs of data from functions
* Can be destructured easily into multiple variables
* Can contain any number of fields -- Use struct when more than 2 or 3 fields
*/

#[derive(Debug)]
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();

    println!("{:?}, {:?}", x, numbers.0); // => 1, 1
    println!("{:?}, {:?}", y, numbers.1); // => 2, 2
    println!("{:?}, {:?}", z, numbers.2); // => 3, 3

    let (employee, acces) = ("Sam", Access::Full);
    println!("{:?}, {:?}", employee, acces); // => "Sam", Full
}
