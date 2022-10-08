use core::num;


fn main() {
    //---------------------------
    //       - Conditional IF
    //                - && logical and
    //                - || logical or
    //                - == is used to check equality
    //                - != is used to check inequality
    //---------------------------

    /* General syntax
    if condition {
        // code
    } else if condition {
        // code
    } else {
        // code
    }
    */
    let some_number = 5;
    if some_number < 5 {
        println!("{} is less than 5", some_number);
    } else if some_number > 5 {
        println!("{} is greater than 5", some_number);
    } else {
        println!("{} is equal to 5", some_number);
    }

    let marks = 40;
    if marks >= 90 && marks <= 100 {
        println!("A+");
    } else if marks >= 80 && marks < 90 {
        println!("A");
    } else if marks >= 70 || marks < 80 {
        println!("B");
    } 

    //----------------------------
    //       - Nested IF
    //----------------------------
    let some_number = 5;
    if some_number < 10 {
        println!("{} is less than 10", some_number);
        if some_number > 5 {
            println!("{} is greater than 5", some_number);
        }
    }
    
    //----------------------------
    //       - Read from user
    //----------------------------
    println!("Enter a number");
    let mut some_number = String::new();
    std::io::stdin().read_line(&mut some_number).expect("Failed to read line");
    let some_number: i32 = some_number.trim().parse().expect("Please type a number");
    if some_number < 10 {
        println!("{} is less than 10", some_number);
    } else {
        println!("{} is greater than 10", some_number);
    }

    //----------------------------
    //       - If let statement
    // let variable_num = if condition{
    //     // code
    // } else {
    //     // code
    // };
    //----------------------------
    let value = if true {10} else {20}; // both should have same type
    println!("Value is {}", value);

    // Example - 2
    let mark = 90;
    let grade = if mark >= 90 && mark <= 100 {
        "A+"
    } else if mark >= 80 && mark < 90 {
        "A"
    } else if mark >= 70 || mark < 80 {
        "B"
    } else {
        "Fail"
    };
    println!("Grade is {}", grade);

    //----------------------------
    //      - Match statement
    //----------------------------

    /* General syntax
    match variable {
        pattern => code,
        pattern => code,
        pattern => code,
        _ => code
    }
    */
    let some_number = 7;
    // Need default case or match should be exhaustive
    match some_number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        6..=10 => println!("Six to Ten inclusive"),
    
        11 | 12 => println!("Eleven or Twelve"),
        _ => println!("Unknown number")
    }
    
    //----------------------------
    //      - Match with let statement
    //         - return value should be same type
    //----------------------------
    let some_number = 7;
    let some_string = match some_number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6..=10 => "Six to Ten inclusive",
        11 | 12 => "Eleven or Twelve",
        _ => "Unknown number"
    };
    println!("Some string is {}", some_string);

}
