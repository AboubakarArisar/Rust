//There are four types of commens
//1-single line comments
//2-Multi line comments
//3-Inner Doc comments
//4-Outer Doc comments


// Writing a Rust program
fn main() {
    //The line comment is the recommended comment style
    println!("This is a line comment!"); // print hello World to the screen

    /* 
    The block comment is extremely useful for temporarily disabling
    a large chunk of code. /* Block comments can also be /* nested, */ */
    To comment a large block just write in between /* text */
    */
    println!("This is a line comment!"); // print hello World to the screen

     //! This a doc comment that is inside the function   
    //! This comment shows my code inside a module or a function  
    //! Generate docs for the enclosing item
    println!("{} can support {} notation","Doc comment","markdown");
}


  /// This is a Doc comment outside the function
/// Generate docs for the following item.
/// This shows my code outside a module or a function
