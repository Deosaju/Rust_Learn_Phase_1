fn main() {
        // Multiple variables
    let (First_Number, Second_Number) = (10, 20);
    println!("First Number: {} and Second Number: {}", First_Number, Second_Number);

    //Readability of numbers
    let number = 1_000_000;
    println!("Number: {}", number);

    // Integer Overflow
    let number_overflow : u8 = 255; //256 will overflow 
    println!("Number: {}", number_overflow);

    // Formatting
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Operations in different types
    let n1 : i32 = 10;
    let n2 : i64 = 20;

    //will change the type of n2 to i32 only here
    let n3 = n1 + n2 as i32;

    println!("n3: {}", n3);

    /* -------------------
    Shadowing
    ------------------- */
    //Shadowing is updating or redefining a variable that was already defined, new value is only available in the scope of the variable

    let s = 10;
    let s = s + 10;
    println!("s: {}", s);

    let mut s = 10;
    let s = s + 10;
    println!("s: {}", s);

    let s = 32;
    println!("s: {}", s);

    let s = "Hello";
    println!("s: {}", s);

    //-----------------------
    // Constants : Constants are immutable by default and cannot be shadowed
    //-----------------------
    const MAX_PAIN: u32 = 100_000;
    println!("MAX_PAIN: {}", MAX_PAIN);

}
