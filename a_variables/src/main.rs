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
    advanced_destructuring_ex();
    very_advanced_destructuring_ex();
    destructuring_ex();
    destructuring_with_underscore_ex()

}

fn immutable_variable_ex() {
    // Immutable Variable : A variable whose value cannot be changed after it is bound to a value.
    println!("Immutable Variable Example");
    let x = 5;
    println!("x is : {}", x);
    // x = 6; // This will not compile
}

fn mutable_variable_ex() {
    // Mutable Variable : A variable whose value can be changed after it is bound to a value.
    println!("Mutable Variable Example");
    let mut x = 5;
    println!("x is : {}", x);
    x = 6;
    println!("x is : {}", x);
    // This will print x + 6
}

fn constant_ex() {
    // Constant : A variable whose value cannot be changed after it is bound to a value.
    println!("Constant Example");
    println!("Life lessons are worth it: {}", LIFE_LESSONS_ARE_WORTH_IT);
    println!("Life lessons are not worth it: {}", LIFE_LESSONS_ARE_NOT_WORTH_IT);

    // Output : Life lessons are worth it: 180
    //          Life lessons are not worth it: 0
}

fn shadowing_ex() {
    // Shadowing : A variable can be redefined with the same name. The new variable shadows the previous variable. 
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
    // Scope : The scope is the range within the program for which an item is valid. 
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
    // Type Annotation : The type of a variable can be explicitly defined. 
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
    // Mutable vs Shadowing : Shadowing is different from making a variable mutable. 
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
    // Destructuring : Destructuring is a way to break down a complex data structure into simpler parts. 
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

fn advanced_destructuring_ex() {
    // Advanced Destructuring : Destructuring can be used to ignore some values in a tuple.
    println!("Advanced Destructuring Example");
    let (first, ..) = (1, 2, 3);
    println!("The value of first is: {}", first);

    let (.., last) = (1, 2, 3);
    println!("The value of last is: {}", last);

    // Incorrect Destructuring
    // let (.., mid, ..) = (1, 2, 3);
    println!();
}

// Output : Advanced Destructuring Example
//           The value of first is: 1
//           The value of last is: 3
//           The value of first is: 1
//           The value of last is: 3
//           The value of first is: 1
//           The value of last is: 3
//           The value of first is: 1
//           The value of last is: 3..........


fn very_advanced_destructuring_ex() {
    println!("Very Advanced Destructuring Example");
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };
    let Point { x, y } = point;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // Incorrect Destructuring
    // let Point { a, b } = point;
    println!();
}
// Output : Very Advanced Destructuring Example
//           The value of x is: 1
//           The value of y is: 2
//           The value of x is: 1
//           The value of y is: 2
//           The value of x is: 1
//           The value of y is: 2.........

