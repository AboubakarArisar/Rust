fn main(){
   println!("- Passing a string literal"); 
   concatenate(" Rust ", " Programming "); 
   println!("- Passing an integer"); 
   concatenate(10 as i32, 1 as i32);
   
}
use std::fmt::Display;
fn concatenate<T:Display>(t:T, s:T){
   let result = format!("{}{}", t , s);
   println!("{}", result);
}
