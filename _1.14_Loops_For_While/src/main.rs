fn main() {
    //----------------------------
    //      - Loops
    //            - Infinite loop
    //            - while loop
    //----------------------------
    /*
    let mut counter = 0;
    loop {
        println!("Counter is {}", counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let number = 5;
    let mut guess = false;
    while !guess {
        println!("Guess the number");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Please type a number");
        if input == number {
            println!("You guessed the number");
            guess = true;
        } else if input < number {
            println!("Too small");
        } else {
            println!("Too big");
        }
    }

    // Example -2
    println!("Please Enter a number i will tell the next number divisibl by both 2, 5");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let mut number: i32 = number.trim().parse().expect("Please type a number");
    let mut divisible_by = false;
    while divisible_by != true {
        number = number + 1;
        if number % 2 == 0 && number % 5 == 0 {
            println!("{} is divisible by both 2 and 5", number);
            divisible_by = true;
        }
    }
    */

    //---------------------------
    //      - For loop
    //---------------------------
    /*
    let vector = vec![1, 2, 3, 4, 5];
    for i in 0..5 {
        println!("Number is {}", vector[i]);
    }
    for i in vector {
        println!("Number is {}", i);
    }
    // println!("{:?}", vector); // the value is moved , so we can't use it again

    let vector = vec![1, 2, 3, 4, 5];
    for i in &vector {
        println!("Number is {}", i);
    }
    println!("{:?}", vector); // the value is not moved , so we can use it again

    // Example - 2
    let vector = vec![1, 2, 3, 4, 5];
    for i in vector.iter() {
        println!("Number is {}", i);
    }
    println!("{:?}", vector); // the value is not moved , so we can use it again
     */

    let mut vector = vec![1, 2, 3, 4, 5];
    for i in vector.iter_mut() { // we can use iter_mut() to change the value of the vector
        *i = *i + 1; // when we use iter_mut we need to dereference the value to change it
        println!("Number is {}", i);
    }
    println!("{:?}", vector); // the value is not moved , so we can use it again

    let mut vector = vec![1, 2, 3, 4, 5];
    for i in &mut vector { // we can use &mut to change the value
        *i = *i + 1; // when we use iter_mut we need to dereference the value to change it
        println!("Number is {}", i);
    }
    println!("{:?}", vector); // the value is not moved , so we can use it again
}
