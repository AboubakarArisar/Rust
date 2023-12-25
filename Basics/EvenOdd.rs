fn test(_a: i32) {
    // Write code here
    let res = _a % 2;
    match res {
        0 => print!("Number {} is even", _a),
        1 => print!("Number {} is odd", _a),
        _ => print!("Invalid result"), 
    };
}
