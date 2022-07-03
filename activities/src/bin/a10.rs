// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(is_big: bool) {
    if is_big {
        println!("its big");
    } else {
        println!("its small");
    }
}

fn check_if_big(my_num: i32) -> bool {
    let is_big: bool = matches!(my_num, n if n > 100);
    is_big
}

fn main() {
    let my_num = 10423423;

    let is_big = check_if_big(my_num);

    print_message(is_big)
}
