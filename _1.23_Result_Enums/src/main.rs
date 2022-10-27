// Result Enum
    // -Result is an enum that can be either Ok or Err
    // - contains the success value
    // -Err contains the error value

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    /*
    if divisor == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(dividend / divisor)
    }
    */
    match divisor {
        0.0 => Err(String::from("Division by zero")),
        _ => Ok(dividend / divisor),
    }
}

fn main() {
    println!("{:?}", division(9.0, 3.0));
    println!("{:?}",division(9.0,0.0));

    // Vectors and Result
    let v = vec![1, 2, 3];
    let reuslt = match v.get(99) {
        Some(x) => Ok(x),
        None => Err("The Number doesn't exist"),
    };
    println!("{:?}", reuslt);
}
