//------------------------
//          Generics
//               - Why Generics?
//------------------------

/*
fn square(x: i32) -> i32 {
    x * x
}
fn square_f64(x: f64) -> f64 {
    x * x
}

fn main() {
    let x = 5;
    let y = square(x);
    println!("y = {}", y);

    let x = 5.0;
    let y = square_f64(x);
    println!("y = {}", y);

}

*/

// Here there is a lot of duplication in the code. We have to write the same code for different types.

// Generics allow us to write code that can be used for different types.

fn square <T: std::ops::Mul<Output = T> + Copy> (x: T) -> T {
    x * x
}

//std::ops::Mul<Output = T> + Copy
//std::ops::Add<Output = T> + Copy
//std::ops::Sub<Output = T> + Copy
//std::ops::Div<Output = T> + Copy

// Alternative syntax
// fn square <T> (x: T) -> T where T: std::ops::Mul<Output = T> + Copy{// Code}

fn main() {
    let x = 5;
    let y = square(x);
    println!("y = {}", y);

    let x = 5.5;
    let y = square(x);
    println!("y = {}", y);

}