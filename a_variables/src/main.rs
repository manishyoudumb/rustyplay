const LIFE_LESSONS_ARE_WORTH_IT: u32 = 180;
const LIFE_LESSONS_ARE_NOT_WORTH_IT: u32 = 0;

fn main() {
    immutable_variable_ex();
    mutable_variable_ex();
    constant_ex();
    shadowing_ex();
    scope_ex();
    type_annotation_ex();
    mutable_vs_shadowing_ex();

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
    // Output : The value of y is: 300
    //          The value of x in inner scope is: 200
    //          The value of x in outer scope is: 200


fn type_annotation_ex() {
        println!("Type Annotation Example");
        let x: i32 = 15;
        println!("The value of i32 x is: {}", x);
        let x: u8 = 15;
        println!("The value of u8 x is: {}", x);
        println!();
}
    
//  Output :  Type Annotation Example
//            The value of i32 x is: 15
//            The value of u8 x is: 15



fn mutable_vs_shadowing_ex() {
    println!("Mutable vs Shadowing Example");
    // Can change the value of the var but not the type
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    // Can change both value and type
    let x = false;
    println!("The value of x is: {}", x);
    println!();
}

// Output : Mutable vs Shadowing Example
//           The value of x is: 5
//           The new value of x is: 6
//           The value of x is: 7
//           The value of x is: false


fn destructuring_ex() {
    println!("Destructuring Example");
    let (x, y) = (1, 2);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!();
}
    // Output : Destructuring Example
    //           The value of x is: 1
    //           The value of y is: 2


fn destructuring_with_underscore_ex() {
    println!("Destructuring with Underscore Example");
    // Not Recommended. Why? Because it is not idiomatic Rust code.

    let (x, _) = (1, 2);
    println!("The value of x is: {}", x);
    println!();
}

// Output : Destructuring with Underscore Example
//           The value of x is: 1
//           The value of y is: 2
//           The value of x is: 1
//           The value of x is: 1
//           The value of x is: 1
//           The value of x is: 1
//           The value of x is: 1
//           The value of x is: 1 .........