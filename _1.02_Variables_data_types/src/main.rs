fn main() {
    //-------------------------------
    //         Variables in Rust
    //-------------------------------

    //additional information is data types
    let x: i32 = 1; //This is a variable
    // let x : i64 = 10; //This is also variable

    println!("The value of x is: {}", x); //This will print the value of x

    //x = 5; //This will give an error because x is immutable

    //Mutable variables

    let mut y = 5; //This is a mutable variable
    println!("The value of y is: {}", y); //This will print the value of y

    /* Rules for variables
    1. Variable names can contain only alphabets, numbers, and underscores
    2. Variable names cannot start with a number
    3. Variable names are case sensitive
    4. Variable names cannot be a reserved keyword
    */

    /* Data types in Rust
     Scalar data types - Integers, Floating-Point Numbers, Booleans, Characters
     Compound data types - Tuples, Arrays
    */

    // Integer data types
    // i8, i16, i32, i64, i128, isize Formula for i8 is -2^(i-1) to 2^(i-1) - 1
    // u8, u16, u32, u64, u128, usize

    let a: i8 = 10; //This is a signed integer
    let b: u8 = 10; //This is an unsigned integer

    // The maximum number of i8 is 127
    println!("The maximum number of i8 is {}", i8::MAX);

    // The minimum number of i8 is -128
    println!("The minimum number of i8 is {}", i8::MIN);

    //--------------------------
    // Floating-Point Numbers
    //--------------------------

    let c: f32 = 10.0; //This is a 32-bit floating-point number
    let d: f64 = 10.04; //This is a 64-bit floating-point number

    println!("The value of d is {}", d);

    //--------------------------
    // Boolean
    //--------------------------

    let e: bool = true; //This is a boolean
    println!("The value of e is {}", e);

    let not_equals: bool = 18 != 17 ; // <, >, <=, >=, ==, !=
    println!("The value of not_equals is {}", not_equals);

    //--------------------------
    // Character
    //--------------------------

    let f: char = 'a'; //This is a character
    let g: char = '😀'; //This is also a character
    let three: char = '3'; //This is also a character
    let heart_eyed_cat: char = '\u{288A}'; //This is also a character

    println!("The value of g is {}", g);
    println!("The value of three is {}", heart_eyed_cat);


}