fn main() {
    // String concatenation and Ownership

    let s1 = String::from("Hello");
    let s2 :&str = "World";
    let s3 = s1 + " " + s2; // store the result in location of s1 in heap memory

    // println!("{}", s1);// error: value borrowed here after move
    println!("{}", s3);

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + " " + &s2; // Ownership of s1 is changed from s1 to s3
    println!("{}", s3);

    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = String::from("From Rust");
    let s7 = s4 + " " + &s5 + " " + &s6; // Ownership of s4 is changed from s4 to s7
    println!("{}", s7);

}
