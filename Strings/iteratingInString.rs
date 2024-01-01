fn main() {
  // define a String object
  let str = String::from("Rust Programming");  
  // split on literal
  for token in str.chars(){
      println!("{}", token);
  }
}
