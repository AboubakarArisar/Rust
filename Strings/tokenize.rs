fn main() {
  // define a String object
  let str = String::from("Interactive,course on,Rust,Programming");  
  // split on token
  for token in str.split(","){
      println!("{}", token);
  }
}
