//declare a struct
struct Course {
   code:i32,
   name:String,
   level: Option<String>, 
}
fn main() {
   //initialize
   let course1 = Course  {
      name:String::from("Rust"),
      level:Some(String::from("beginner")),
      code:130
   };
   let course2 = Course  {
      name:String::from("Javascript"),
      level:None,
      code:122
   };
   //access
   println!("Name:{}, Level:{} ,code: {}", course1.name, course1.level.unwrap_or("Level".to_string()), course1.code);
   println!("Name:{}, Level:{} ,code: {}", course2.name, course2.level.unwrap_or("No level defined!".to_string()), course2.code);
}
