const LIFE_LESSONS_ARE_WORTH_IT: u32 = 180;
const LIFE_LESSONS_ARE_NOT_WORTH_IT: u32 = 0;

fn main() {
    immutable_variable_ex();
    mutable_variable_ex();
    constant_ex();
    shadowing_ex();
    scope_ex();

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

fn constant_ex() {
    println!("Constant Example");
    println!("Life lessons are worth it: {}", LIFE_LESSONS_ARE_WORTH_IT);
    println!("Life lessons are not worth it: {}", LIFE_LESSONS_ARE_NOT_WORTH_IT);

    // Output : Life lessons are worth it: 180
    //          Life lessons are not worth it: 0
}

fn shadowing_ex() {
    println!("Shadowing Example");
    let x = 9;
    println!("x is : {}", x);
    let x = x + 1;
    println!("x is : {}", x);
    let x = x * 2;
    println!("x is : {}", x);
    let x = true;
    println!("x is : {}", x);
    // Output : x is : 10
    //          x is : 18
    //          x is : 9
    //          x is : true
}

fn scope_ex() {
    println!("Scope Example");
    //---------------------------------------------- OUTER SCOPE
        let x = 200;
        {
            //------------------------------------------ INNER SCOPE
            let y = 300;
            println!("The value of y is: {}", y);
            println!("The value of x in inner scope is: {}", x);
            //------------------------------------------ INNER SCOPE
        }
        println!("The value of x in outer scope is: {}", x);
    //---------------------------------------------- OUTER SCOPE
}
