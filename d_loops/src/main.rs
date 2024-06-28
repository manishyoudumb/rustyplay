fn main() {
    println!("Hello, world!");
    example1();
}

fn example1() {
    // Doesn't work based on conditionals
    let mut count = 1;
    loop{
        count += 1;
        if count == 5 {
            println!("Hello");
            break;
        }
    }
}