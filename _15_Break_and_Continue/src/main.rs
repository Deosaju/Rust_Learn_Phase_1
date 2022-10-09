fn main() {
    //-------------------------
    // break
    //-------------------------

    // When break statement is encountered, the loop is exited immediately
    let mut var = 0;
    loop {
        var += 1;
        if var == 5 {
            break;
        }
    }
    println!("var = {}", var);

    //-------------------------
    // continue
    //-------------------------

    let mut var = 0;
    let req = loop {
        var += 1;
        if var == 5 {
            continue;
        }
        else if var == 10 {
            break var;
        }
        println!("var = {}", var);
    };
    println!("req = {}", req);
}
