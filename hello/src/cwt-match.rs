fn main() {
    let my_name: &str = "Sam";
    match my_name {
        "Sam" => println!("That is my name"),
        "Sally" => println!("Not my name"),
        "Samantha" => println!("Hi Samantha"),
        _ => println!("Nice to meet you."),
    }
}
