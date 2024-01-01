fn fibonacci(term: i32) -> i32 {
    match term {
        0 =>  0,
        1 =>  1,
        _ => fibonacci(term-1) + fibonacci(term-2),
    }
}
fn main(){
    println!("fibonacci(4)={}",fibonacci(4));
}
