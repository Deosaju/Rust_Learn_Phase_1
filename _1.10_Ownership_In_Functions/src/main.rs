//                    Rust Ownership Rules
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

    
fn main() {
    // Ownership in Functions
    let stack_var = 10;
    stack_func(stack_var);
    println!("stack_var: {}", stack_var);
    
    let heap_var = vec![1, 2, 3];
    heap_func(heap_var);
    // println!("heap_var: {}", heap_var); // Error: value borrowed here after move

    let heap_vars = vec![1, 2, 3];
    heap_func_ref(&heap_vars); // Pass by reference 
    println!("heap_vars: {:?}", heap_vars); // OK: value borrowed here after move

    let mut heap_vars_mut = vec![1, 2, 3];
    heap_func_mut(&mut heap_vars_mut); // Pass by mutable reference
    println!("heap_vars_mut: {:?}", heap_vars_mut); // OK: value borrowed here after move

    // Should not create mutable reference which itself is mutable
    // let mut heap_vars_mut_mut = vec![1, 2, 3];
    // heap_func_mut_mut(&mut heap_vars_mut_mut); // Error: cannot borrow `heap_vars_mut_mut` as mutable more than once at a time
    // println!("heap_vars_mut_mut: {:?}", heap_vars_mut_mut); // Error: value borrowed here after move

    // let heap_vars_mut = vec![1, 2, 3];
    // let mut heap_vars_mut_mut = heap_vars_mut; // Error: cannot borrow `heap_vars_mut` as mutable because it is also borrowed as immutable

    // let mut heap_vars_mut = vec![1, 2, 3];
    // let mut heap_vars_mut_mut = &heap_vars_mut; 
    // We should not be creating a reference which itself is mutable

    // Understang mutable reference and reference being immutable

    
    //---------------------------------
    // Referance being Handy
    //---------------------------------

    let large_data = String::from("Hello World");
    let large_data2 = String::from("Hello India 000000000000000000");
    let huge_data : Vec<&str> = vec![&large_data, &large_data2];
    println!("huge_data: {:?}", huge_data);
    // Two way to merge two strings

}
fn stack_func(var: i32) {
    println!("stack_var: {}", var);
}
fn heap_func(var: Vec<i32>) {
    println!("heap_var: {:?}", var);
}
fn heap_func_ref(var: &Vec<i32>) {
    println!("heap_vars: {:?}", var);
}

fn heap_func_mut(var: &mut Vec<i32>) {
    var.push(5);
    println!("heap_vars_mut: {:?}", var);
}