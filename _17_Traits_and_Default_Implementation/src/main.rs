//--------------------------------------------
//             Traits
//                  - General Explanation
//                  - Default Implementation
//--------------------------------------------

// Traitts tell the Rust compiler about functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic can be any type that has certain behavior.

/*
struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: u32,
}

struct Student {
    name_std: String,
    age : u8,
    sex : char
}

trait General_info {
    fn info (&self) -> (&str, u8, char);
    fn country (&self) -> &str;

}

impl General_info for Person{
    fn info (&self) -> (&str, u8, char) {
        (&(self.name),self.age,self.gender)
    }    
    fn country (&self) -> &str {
        &(self.citizenship)
    }
}

impl General_info for Student{
    fn info (&self) -> (&str, u8, char) {
        (&(self.name_std),self.age,self.sex)
    }
    fn country (&self) -> &str {
        "India"
    }
}
fn main () {

    let person1 = Person {
        name: String::from("Raj"),
        citizenship: String::from("India"),
        age : 25,
        gender : 'M',
        salary : 50000
    };

    let student1 = Student {
        name_std : String::from("Raju"),
        age : 25,
        sex : 'M'
    };

    println!("{} is from {} and is {} years old and is a {}", person1.info().0, person1.country(), person1.info().1, person1.info().2);

    println!("({:?}", person1.info());
    println!("{} is from {} and is {} years old and is a {}", student1.info().0, student1.country(), student1.info().1, student1.info().2);
}
 */

// Default Implementation
// We can provide default implementations for trait methods.
 
struct Circle {
    radius: f64,
}

struct Rectangle {
    length: f64,
    width: f64,
}

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn is_square(&self) -> bool {
        self.perimeter() == 4.0 * self.area().sqrt()
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }

    fn is_square(&self) -> bool {
        self.perimeter() == 4.0 * self.area().sqrt()
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        length: 5.0,
        width: 6.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Circle perimeter: {}", circle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Is rectangle a square: {}", rectangle.is_square());

}