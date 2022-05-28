/* Match is checked by the compiler - If a new possibility is added, you will be notifed when this occurs */

/* else...if is NOT checked by the compiler - If a new possibilityis added, your code may contain a bug */

// Use Match when working with a single variable

fn main() {
    let some_bool: bool = true;
    match some_bool {
        true => println!("true"),
        false => println!("false"),
    }

    let some_number = 5;
    match some_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        //_ is a stand in for anything else you're checking for. The 'default'
        _ => println!("something else"),
    }
}
