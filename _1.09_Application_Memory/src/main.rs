const Max_Value :i32 = 40_000; // This will be stored in the Global Memory

fn main() { 
    // This will be stored in the Stack Memory 
    let (x, y) = (1, 2);
    let sum_of_squares = square_sum(x, y);
    println!("The sum of squares is {}", sum_of_squares);
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    // This will be stored in the Stack Memory above the main function
    let result = square(num1 + num2);
    result
}

fn square(num :i32) -> i32 {
    // This will be stored in the Stack Memory above the square_sum function
    num * num    
}
// Each memory will be released when the function is done executing
