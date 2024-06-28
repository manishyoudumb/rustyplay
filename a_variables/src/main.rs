const LIFE_LESSONS_ARE_WORTH_IT: u32 = 180;
const LIFE_LESSONS_ARE_NOT_WORTH_IT: u32 = 0;

fn main() {
    immutable_variable_ex();
    mutable_variable_ex();
    constant_example();
}

fn immutable_variable_ex() {
    println!("Immutable Variable Example");
    let x = 5;
    println!("x is : {}", x);
    // x = 6; // This will not compile
}

fn mutable_variable_ex() {
    println!("Mutable Variable Example");
    let mut x = 5;
    println!("x is : {}", x);
    x = 6;
    println!("x is : {}", x);
    // This will print x + 6
}

fn constant_example() {
    println!("Constant Example");
    println!("Life lessons are worth it: {}", LIFE_LESSONS_ARE_WORTH_IT);
    println!("Life lessons are not worth it: {}", LIFE_LESSONS_ARE_NOT_WORTH_IT);

    // Output : Life lessons are worth it: 180
    //          Life lessons are not worth it: 0
}