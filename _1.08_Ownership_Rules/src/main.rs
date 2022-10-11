fn main() {
    /* Ownership 
        - Each value in Rust has a variable that’s called its owner.
        - There can only be one owner at a time.
        - When the owner goes out of scope, the value will be dropped.
    */

    // Copy and Move
    let mut num_1 = 32;
    let mut num_2 = num_1;
    println!("The value of num_1 is: {} and The value of num_2 is : {}", num_1,num_2);
    /* In case of integers y will make a copy of x , as it makes seperate memry locations to store themm 
        - Copy trait is implemented for integers
        - Copy trait is implemented for floating point numbers
        - Copy trait is implemented for booleans
        - Copy trait is implemented for characters
        - Rust has two types of Variables
            - Primitive Variables // Cannot be empty and fixed in size , copy trait is implemented for them
                - Integer
                - Float
                - Boolean
                - Character
            - Non Primitives Variables // Can be empty and can grow in size , moved instead of copied
                - String
                - Vector
                - Tuple
                - Struct
                - Enum
                - Pointer
                - Function
                - Trait
                - Array
                - Slice
                - Tuple
        - Copy trait is implemented for all the primitive variables

    */


    let mut s1 = String::from("Hello");
    
    // let mut s2 = s1;  This will change of the ownership of the value of s1 to s2
    /*
    println!("The value of s1 is: {} and The value of s2 is : {}", s1,s2);
        - In case of strings, y will take the ownership of x, and x will be invalid after this
        - This is called move
    */

    let mut s2 = &s1; // This will not change the ownership of the value of s1 to s2, 
                               //creates a reference to s1
    println!("The value of s1 is: {} and The value of s2 is : {}", s1,s2);


    //-----------------------------------------------
    //            Vector Using Ownership
    //-----------------------------------------------

    let mut v1 = vec![1,2,3,4,5];
    let mut v2 = &v1;
    println!("The value of v1 is: {:?} and The value of v2 is : {:?}", v1,v2);

    //-----------------------------------------------
    //               Clone
    //-----------------------------------------------

    let mut s1 = String::from("Hello");
    let mut s2 = s1.clone(); // This will make a copy of s1 and will not change the ownership of s1
    println!("The value of s1 is: {} and The value of s2 is : {}", s1,s2);

    //-----------------------------------------------
    //               Dropping value out of scope
    //-----------------------------------------------
    {
        let name = String::from("Hello");
    }
    // println!("The value of name is: {}", name); // This will give an error as name is out of scope

}
