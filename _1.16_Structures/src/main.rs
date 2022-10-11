fn main() {
    //----------------------------------------------
    //              Structures
    //                - Defining Structs
    //                - Tuple Structs
    //----------------------------------------------

    /*
    let student1 = Student{
        name: String::from("John"),
        age : 20,
        marks: 90,
        grade : 'A',
    };
    let student2 = Student{
        name: String::from("Mary"),
        age : 21,
        marks: 80,
        grade : 'B',
    };
    let student4 = Student{
        name: String::from("Peter"),
        ..student1 // will copy the remaing data from passed struct
    };

    // Making Mutable Struct Instance
    let mut student4 = Student{
        name: String::from("Peter"),
        ..student1 // will copy the remaing data from passed struct
    };
    student4.name = String::from("Peter");


    println!("Student 1 Details: {},{},{},{}", student1.age, student1.name, student1.marks, student1.grade);
    println!("Student 1 Details: {}", student1.display());
    println!("Student 2 Details: {}", Student::display2(&student2));
    println!("Student 4 Details: {}", student4.display());

    let student3 = Student::new();
    println!("Student 3 Details: {}", student3.display());
     */

    //----------------------------------------------
    //              Tuple Structs
    //----------------------------------------------

    let student6 = Numbers(1,2); 
    println!("Student 6 Details: {},{}", student6.0, student6.1);

    println!("Greater, And lesser: {},{}", student6.greater(), student6.lesser());

}

/*
struct Student {
    name: String,
    age: u8,
    marks: u8,
    grade: char,
}

impl Student { // Functions that operate on object of Structure
    fn display(&self) -> f32{
        self.marks as f32 / 100.0
    }
    fn display2(&self) -> f32{
        self.marks as f32 / 200.0
    }
    fn new() -> Self{
        Student{
            name: String::from(""),
            age: 0,
            marks: 110,
            grade: ' ',
        }
    }
}
 */

// Tuple Structs
struct Numbers(i32, i32);

impl Numbers{
    fn greater(&self) -> i32{
        if self.0 > self.1 {
            self.0
        } else {
            self.1
        }
    }

    fn lesser(&self) -> i32{
        if self.0 < self.1 {
            self.0
        } else {
            self.1
        }
    }

}