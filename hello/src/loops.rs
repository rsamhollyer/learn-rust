fn main() {
    let mut iter: i32 = 3;
    loop {
        println!("Hello, world!");
        println!("{:?}", iter);
        iter -= 1;
        if iter == 0 {
            break;
        }
    }
}
