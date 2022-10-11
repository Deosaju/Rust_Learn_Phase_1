fn main() {
    // Scalar data types and Compound data types
    //------------------
    //  Tuple - a collection of values of different types, fixed length, heterogeneous, immutable 
    //        - destructuring of tuple
    //------------------

    let tup: (&str ,i32, f64, u8) = ("my_salary",500, 6.4, 1);
    println!("tup: {}", tup.0);

    //destructuring of tuple
    let (x, y, z, w) = tup;
    println!("x: {}, y: {}, z: {}, w: {}", x, y, z, w);
    
    // indexing of tuple
    println!("tup.0: {}", tup.0);

    // nested tuple
    let nested_tuple = (1, 2, (3, 4) , "hello");
    println!("nested_tuple.2.1: {}", nested_tuple.2.1);

    let empty_tuple = ();

    /*---------------Arrays-------------*/
    // Arrays - a collection of values of same type, fixed length, homogeneous, immutable
    //        - indexing of array
    //        - array declared with [type; length] syntax
    //        - empty array with zeros - [0; 5]
    //        - array of strings - ["hello", "world"]
    //        - Slice - a reference to a contiguous sequence of elements in a collection

    let arr= [1, 2, 3, 4, 5];
    println!("arr[0]: {}", arr[0]);

    // array of 5 elements, all initialized to 0
    let arr_of_zero = [0; 5];
    println!("arr_of_zero: {:?}", arr_of_zero);

    let mut arr_new = [1, 2, 3, 4, 5];

    // updating array
    arr_new[0] = 10;
    println!("arr[0]: {}", arr_new[0]);

    // string array
    let arr_str = ["hello", "world"];
    println!("arr_str[0]: {}", arr_str[0]);

    // unknown length array
    let arr_unknown = ["unknown"; 5];
    println!("arr_unknown: {:?}", arr_unknown);
     
    //similarly for char array
    let arr_char = ['a', 'b', 'c'];
    println!("arr_char: {:?}", arr_char);

    // slices are references to a contiguous sequence of elements in a collection
    let mut slices = [1, 2, 3, 4, 5];
    let slice = &slices[0..3];
    println!("slice: {:?}", slice);

    let eql_slice = &slices[0..=3];
    println!("eql_slice: {:?}", eql_slice);

    //slices[0] = 10; // error: cannot assign twice to immutable variable `slices`

    //----------------
    //  Functions on array
    //----------------

    println!("The array uses {} size", std::mem::size_of_val(&arr));

    //get function
    println!("arr.get(0): {:?}", arr.get(1)); // answer returns some(value) or none


}
