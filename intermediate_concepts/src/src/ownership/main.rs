/* Managing Memory
Programs must track memory
+ If they fail to do so, a 'leak' occurs
Rust utilizes an 'ownership' model to manage memory
+ The 'owner' of memory is responsible for cleaning up the memory
Memory can either be 'moved' or 'borrowed
 */

/* Move Example */

enum Light {
    Bright,
    Dull,
}

// fn display(light: Light) {
//     match light {
//         Light::Bright => println!("The light is bright!"),
//         Light::Dull => println!("The light is dull!"),
//     }
// }

// fn main() {
//     let dull = Light::Dull;
//     let number: i32 = 42;
//     display(dull);
//     // 'dull' gets deleted in memory here, that's why an error occurs when the program tries to use it again
//     // display(dull);
// }

/* Borrow Example */

fn display(light: &Light) {
    match light {
        Light::Bright => println!("The light is bright!"),
        Light::Dull => println!("The light is dull!"),
    }
}

// The ampersand (&) is used to borrow a value -- it's a reference to a value
fn main() {
    let dull = Light::Dull;
    display(&dull);
    display(&dull);
}
