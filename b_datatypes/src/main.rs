fn main() {
scalar_type();
compund_type();
}

/* Scalar Type
> represents a single value
>  4 primary types : integer, floating-point number, Numeric operations, boolean, or character
*/
fn scalar_type() {
/*Integer Types
   > An integer is a number without a fractional component.
   > Rust has signed and unsigned integers.
   > Signed integers can store positive and negative values.
   > Unsigned integers can store only positive values.
   > The number in the type name indicates the number of bits the integer type uses.
*/
    let mut x:i8 = 42; // i8 : signed 8-bit integer
    let mut y:u8 =42;  // u8 : unsigned 8-bit integer

    let mut x:i16 = 42; // i16 : signed 16-bit integer
    let mut y:u16 =42;  // u16 : unsigned 16-bit integer

    let mut x:i32 = 42; // i32 : signed 32-bit integer
    let mut y:u32 =42;  // u32 : unsigned 32-bit integer

    let mut x:i64 = 42; // i64 : signed 64-bit integer
    let mut y:u64 =42;  // u64 : unsigned 64-bit integer

    let mut x:i128 = 42; // i128 : signed 128-bit integer
    let mut y:u128 =42;  // u128 : unsigned 128-bit integer

    let mut x:isize = 42; // isize : signed integer
    let mut y:usize =42;  // usize : unsigned integer

/*Floating-Point Types
    > A floating-point number is a number with a decimal point.
    > Rust has two floating-point types: f32 and f64.
    > f32 is a single-precision float, and f64 is a double-precision float.
    > f64 is the default type for floating-point numbers because it is more precise.
*/

    let mut x:f32 = 42.0; // f32 : 32-bit floating-point number
    let mut y:f64 = 42.0; // f64 : 64-bit floating-point number

/*Numeric Operations
    > Rust supports the basic arithmetic operations: addition, subtraction, multiplication, and division.
    > Rust also supports the remainder operation, which calculates the remainder of the division of two numbers.
    > The remainder operation is also called the "modulo operation."
    > Rust also supports the power operation, which calculates the power of a number.
*/

    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder
    let power = 2u32.pow(4); // power

/*Boolean Type
    > A boolean type represents a logical value.
    > A boolean type can have one of two values: true or false.
*/

    let t = true;
    let f: bool = false;

/*Character Type
    > A character type represents a single Unicode character.
    > Rust's char type is four bytes in size and represents a Unicode Scalar Value.
    > A Unicode Scalar Value is a unique number that represents a Unicode character.
    > Rust's char type is not a single byte like other programming languages.
*/

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of i8 x is: {}", x);
    println!("The value of u8 x is: {}", y);
    println!("The value of i16 x is: {}", x);
    println!("The value of u16 x is: {}", y);
    println!("The value of i32 x is: {}", x);
    println!("The value of u32 x is: {}", y);
    println!("The value of i64 x is: {}", x);
    println!("The value of u64 x is: {}", y);
    println!("The value of i128 x is: {}", x);
    println!("The value of u128 x is: {}", y);
    println!("The value of isize x is: {}", x);
    println!("The value of usize x is: {}", y);
    println!("The value of f32 x is: {}", x);
    println!("The value of f64 x is: {}", y);
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);
    println!("The value of power is: {}", power);
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
    println!();

}

/*Compound Type
    > Compound types can group multiple values into one type.
    > Rust has two primitive compound types: tuples and arrays.
    > Compound types can store multiple values of different types.
    > Compound types can store multiple values of the same type.
*/

fn compound_type() {
/*Tuple Type
    > A tuple is a collection of values of different types.
    > A tuple is a fixed-size collection of values.
    > A tuple can have any number of elements.
    > A tuple can have elements of different types.
    > A tuple can have elements of the same type.
    > A tuple is created by enclosing the values in parentheses.
    > A tuple can be destructured to access its elements.
    > Allocated on the stack as it has a fixed size and is immutable.
    > Cannot grow or shrink in size. 
*/

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    println!();

/* Array Type

 */
}



