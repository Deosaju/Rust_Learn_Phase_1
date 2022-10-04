fn main() {
    /*-----------------Funtions----------------- */
    // Functions are used to store blocks of code for re-use
    // Functions are declared using the fn keyword
    // Functions can have parameters and a return value , or neither of them
    // Functions can be called from anywhere in the program

    // Function with no parameters and no return value
    say_hello();

    // Function with parameters and no return value
    input_para("Deo");

    let mut name :&str = "DeoSaju";

    input_para(name);

    // Function with parameters and return value
    let return_name :&str = return_input("DeoSaju");
    println!("Return name is {}",return_name);
    
    // Functions with multiple Output
    let (name ,age) = multiple_output("DeoSaju", 25);

    println!("Name is {} and Age is {}",name,age);
    
    let result = multiple_output("DeoSaju", 25);
    println!("Name is {} and Age is {}",result.0,result.1);

    // Code blocks are used to create new scopes, allowing you to group
    let x = {
        // This variable is only accessible in this code block
        let x = 5;
        println!("x is {}",x);
        format!("{} is {}",x,"DeoSaju")
    };
    println!("x is {}",x);

    //--------------------------
    // User Input
    //--------------------------

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let n :char = input.trim().parse().expect("Please type a number!"); // change the type of input to any type
    println!("You typed {}",n);
}

// Basic function
fn say_hello() {
    println!("Hello World");
}

// function with parameters
fn input_para(name: &str) {
    println!("Hello {}", name);
}

// function with parameters and return value
fn return_input(name :&str) -> &str {
    name // return value of the function is the last expression in the function body without a semicolon at the end
}

// Functions with multiple Output
fn multiple_output(name :&str,age :i32) -> (&str, i32) {
    ("Deo", 2)
}   