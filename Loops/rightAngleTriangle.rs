fn test(n:i32) {
    // define a nested for loop
    for i in 0..n { //outer loop
        for j in 0..i + 1 { // inner loop
            print!("&");
    } 
    println!("");
    }
}
fn main(){
    println!("Right angled triangle when n = 5 ");
    test(5);
    println!("Right angled triangle when n = 6 ");
    test(6);
}
