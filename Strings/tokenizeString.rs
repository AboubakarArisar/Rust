fn main() {
  // define a String object
  let str = String::from("Rust Programming"); 
  // split on whitespace
  for token in str.split_whitespace(){
      println!("{}", token);
  }
}
