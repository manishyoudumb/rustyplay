fn main() {
scalar_type();
}

/* Scalar Type
> represents a single value
>  4 primary types : integer, floating-point number, boolean, or character
*/
fn scalar_type() {
    // Integer Types
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

    // Floating-Point Types
    let mut x:f32 = 42.0; // f32 : 32-bit floating-point number
    let mut y:f64 = 42.0; // f64 : 64-bit floating-point number
    
}