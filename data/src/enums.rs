/* Enumeration

- Data that can be one of multiple differnt possibilities
    - Each possibility is called a variant
- Provies information about your program to the compiler
    - More robust programs
*/
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }
}

fn main() {
    which_way(Direction::Up);
    which_way(Direction::Down);
    which_way(Direction::Left);
    which_way(Direction::Right);
}
