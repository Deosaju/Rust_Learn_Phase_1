/* Generic Structures */

struct Point <T> {
    x: T,
    y: T,
}
struct Point2 <T, U> {
    x: T,
    y: U,
}

// impl <T:std::fmt::Debug + std::fmt::Display , U:std::fmt::Debug + std::fmt::Display > Point2<T, U> {
//     fn println(&self){
//         println!("{},{}",&self.x, &self.y);
//     }
// }

impl <T, U> Point2<T, U> 
where T:std::fmt::Debug + std::fmt::Display , 
      U:std::fmt::Debug + std::fmt::Display{
    fn println(&self){
        println!("{},{}",&self.x, &self.y);
    }
}

fn main(){

    let p1 = Point {x: 10, y: 20};

    println!("p1.x = {}", p1.x);

    let p2 = Point {x: 10.5, y: 20.5};

    println!("p2.x = {}", p2.x);

    // let p3 = Point {x: 10, y: "World"}; Will throw error since x is of type i32 and y is of type &str
    let p3 = Point2 {x: 10, y: "World"};
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    //----------------------------
    //          impl Functions for Generics
    //----------------------------
    
    // Vector type of vector is generic
    
}
