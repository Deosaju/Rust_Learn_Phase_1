fn main() {
    //Comments in Rust are done with two slashes
    //This is a single line comment

    /*This is a multiline 
    command */

    //Print command
    println!("Hello, world!");

    //learning variables    
    let x = 10; //This is a variable
    println!("The value of x is: {}", x); //This will print the value of x

    println!("The first Name is: {} and my last Name is {}", "John","doe"); //This will print the first name

    //printing in same line
    print!("Deo");
    println!("Saju");

    //printing in multiple lines
    println!("Deo
    Saju");

    //Using Escape Sequences
    println!("My Name : \n Deo\tSaju"); //This will print Deo and Saju in same line with a tab space

    //Discarding backslash
    println!("My Name : \\n Deo\\Saju"); //This will print Deo\Saju

    //carriage return
    println!("My Name : \r Deo Saju"); //This will print Deo Saju

    //learning the positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "John", "India", "code");

    //learning the named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

}
