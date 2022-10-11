//-----------------------------
// 1. enum
//   - enum is a type that can be one of a few different variants
//   - each variant can have different types and amounts of associated data
//   - Enum with attached data
//   - Vectors Containing Enum Values
//-----------------------------


/* General Syntax */

enum Conveyance {
    Car,
    Train,
    Air,    
}
enum Paths {
    Car = 10,
    Train = 20,
    Air = 30, 
} // enum with values

enum Way {
    Car(i32),
    Train(i32),
    Air(i32), 
} // enum with attached data



impl Conveyance {
    fn get_speed(&self) -> i32 {
        match self {
            Conveyance::Car => 60,
            Conveyance::Train => 30,
            _ => 0,
        }
    }
}

impl Paths {
    fn get_speed(&self) -> i32 {
        match self {
            Paths::Car => 60,
            Paths::Train => 30,
            _ => 0,
        }
    }
}

impl Way {
    fn get_speed(&self) -> i32 {
        match self {
            Way::Car(speed) => *speed,
            Way::Train(speed) => *speed,
            Way::Air(speed) => *speed,
        }
    }
}

#[derive(Debug)]
enum Value{
    Integer(i32),
    Float(f64)
}

fn main() {
    let x = Conveyance::Car;
    let y = Conveyance::Train;
    let z = Conveyance::Air;
    let a = Paths::Air;
    let b = Way::Air(100);



    println!("x = {}", x as i32); // need to cast to proper type to print
    println!("y = {}", y as i32);
    println!("a = {}", a as i32);

    println!("x speed = {}", b.get_speed());

    
    let some_vector = vec![Value::Integer(1), Value::Float(2.0)];
    println!("the valueo of some_vector is {:?}", some_vector[0]);

    for i in some_vector {
        match i {
            Value::Integer(x) => println!("Integer: {}", x),
            Value::Float(x) => println!("Float: {}", x),
        }
    }
}