//  Option Enum
//      - Used to capture failure of a part or segment of a program
//      - Two varient is used to capture the success or failure of a program
//      - The None varient is used to capture the failure of a program
//      - The Some varient is used to capture the success of a program
// Thourgh understanding of the Option Enum is important for optional values and error handling

// Basic Syntax
// enum Option<T> {
//     Some(T),
//     None,
// }

// Example



fn main() {
    let mut disease = Option::None;
    disease = Option::Some("Covid-19");

    match disease {
        Option::Some(disease) => println!("Disease: {}", disease),
        Option::None => println!("No Disease"),
    }

    let s1:Option<&str> = Some("Covid-18");
    println!("Disease: {:?} {}",s1, s1.unwrap());

    let f1:Option<f64> = Some(3.14);
    let mut f2 = 16.5;
    f2 = f1.unwrap() + f2;
    println!("f2: {}", f2);

    let v1:Option<Vec<i32>> = Some(vec![1,2,3,4,5]);

    let p1 = Person{
        name:String::from("John"),
        age: 20,
    };
    let someone = Some(p1);
    
    // using Options as function arguments
    let number = Some(7);
    let letter:Option<i32> = None;
    if square(letter) != None {
        println!("Square: {}", square(letter).unwrap());
    }else{
        println!("No number");
    }
    if square(number) != None {
        println!("Square: {}", square(number).unwrap());
    }else{
        println!("No number");
    }
}
struct Person {
    name: String,
    age: u8,
}

fn square (x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x * x),
        None => None,
    }
}