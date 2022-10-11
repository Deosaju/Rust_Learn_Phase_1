/*                      Why Rust is Memeory Safe                */
// Reference Rules:
//           - one Mutable reference 
//           -  Many immutable references are allowed
//           - Immutable and mutable does not coexist

use std::collections::BinaryHeap;

fn main() {
    let mut s = String::from("Hello");
    let ref1 = &mut s;
    // let ref2 = &mut s; // Data race - error // One mut ref variable for data stability
    println!("{}", ref1);

    let mut vector = vec![1, 2, 3, 4, 5];
    let ref3 = &vector;
    let ref4 = &vector; // We can have multiple immutable references
    println!("{} {}", ref3[0], ref4[0]);
    
    // let mut vector1 = vec![1, 2, 3, 4, 5];
    // let ref5 = &vector1;
    // let ref6 = &vector1; // We can have multiple immutable references
    // let mut ref7 = &mut vector1; // We can have one mutable reference
    // println!("{} {} {}", ref5[0], ref6[0], ref7[0]);
    

    //------------------------------------
    //             Scope of Ref
    //------------------------------------

    let mut heap_num = vec![1, 2, 3, 4, 5];
    let ref8 = &heap_num;
    let ref9 = &heap_num;
    println!("{} {}", ref8[0], ref9[0]);  // after println! ref8 and ref9 are dropped

    let ref10 = &mut heap_num;
    println!("{}", ref10[0]); // after println! ref10 is dropped

    //------------------------------------
    //              Data should not change when immutable reference is in scope
    //------------------------------------
    let mut heap_num2 = vec![1, 2, 3, 4, 5];
    let ref11 = &heap_num2;
    let ref12 = &heap_num2;
    heap_num2.push(6); // error - data changed when immutable reference is in scope
    // println!("{} {}", ref11[0], ref12[0]);  // after println! ref8 and ref9 are dropped
    // Because ref11 and ref12 are not in scope anymore

    let mut heap_num2 = vec![1, 2, 3, 4, 5];
    let ref11 = &heap_num2;
    let ref12 = &heap_num2;
    println!("{} {}", ref11[0], ref12[0]);  // after println! ref8 and ref9 are dropped
    heap_num2.push(6); // no error - the immutable reference is not in scope anymore
    
}
 