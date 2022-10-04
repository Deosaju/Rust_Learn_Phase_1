fn main() {
    //--------------------
    //          Vector Data Type -  Collection of similar elements that do not have a fixed size
    //                          - Contiguous in memory block
    //                          - Slides of vector are valid
    //--------------------

    // 1. Create a vector
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    println!("Vector: {:?}",vect);

    // 2. Accessing elements
    println!("First element: {}",vect[0]);

    // 3. Updating elements
    vect[0]=100;
    println!("Vector: {:?}",vect);

    // 4. Initializing a vector with a default value
    let mut vect=vec![0;10];
    println!("Vector: {:?}",vect);

    // 5. Vectors of strings
    let mut vect=vec!["Hello","World"];
    println!("Vector: {:?}",vect);

    // 6. Slicing a vector
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    let slice=&vect[0..3];
    println!("Slice: {:?}",slice);
    
    let slice=&vect[0..=3]; //referances of vect[0] to vect[3]
    println!("Slice: {:?}",slice);

    // 7. Length of a vector
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    println!("Length: {}",vect.len());

    //8. Pushing elements to a vector
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    vect.push(11);
    println!("Vector: {:?}",vect);

    //9. Popping elements from a vector
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    vect.pop();
    println!("Vector: {:?}",vect);

    //10. get() method
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    println!("Element: {:?}",vect.get(9));

    //11. Removing elements from a vector
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];    
    vect.remove(0);
    println!("Vector: {:?}",vect);

    //12. Contains method
    let mut vect=vec![1,2,3,4,5,6,7,8,9,10];
    println!("Contains 5: {}",vect.contains(&5)); // note the & before 5 as contains() takes a reference to the element to be searched



    

}
