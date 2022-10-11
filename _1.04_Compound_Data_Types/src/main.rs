fn main() {
    //--------------------
    //       Strings
    //             - string slices(&str)
    //--------------------

    let some_string = "Hello, world!";
    println!("some_string: \'{}\'", some_string);

    /* Variable Length Strings
        - Length will change
        - add, remove, or change characters
        - Formatting and concatenation
    */
    let mut growable_string = String::from("This string will grow");
    println!("growable_string: \'{}\'", growable_string);

    growable_string.push('s');

    growable_string.push_str(" and grow some more"); // will add this string to the end of the string
    println!("growable_string: \'{}\'", growable_string);

    growable_string.pop();// will remove the last character
    println!("growable_string: \'{}\'", growable_string);

    let length_string = growable_string.len(); // will return the length of the string  
    println!("length_string: \'{}\'", length_string);

    let empty:bool = growable_string.is_empty(); // will return true if the string is empty
    println!("empty: \'{}\'", empty);

    let capacity = growable_string.capacity(); // will return the capacity of the string
    println!("capacity: \'{}\'", capacity);

    let contains = growable_string.contains("deo"); // will return true if the string contains the substring
    println!("contains: \'{}\'", contains);

    growable_string.push_str("    ");
    println!("growable_string: \'{}\'", growable_string);

    let trimmed = growable_string.trim(); // will remove the whitespace from the beginning and end of the string
    println!("trimmed: \'{}\'", trimmed);

    let replaced = growable_string.replace("grow", "shrink"); // will replace the first string with the second string
    println!("replaced: \'{}\'", replaced);

    //Converting intergers to strings
    let number = 10;
    let number_string = number.to_string();
    println!("number_string: \'{}\'", number_string);
    
    //Concate strings
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");
    let string3 = string1 + &string2; // string1 has been moved here and can no longer be used
    println!("string3: \'{}\'", string3);


}
