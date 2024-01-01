// declare a struct
struct Course {
   name: String,
   level:String,
   code: i32,
}
impl Course {
   // static method
   fn my_static_method(n: String, l: String, c:i32) -> Course {
      Course { 
      name: n, 
      level:l,
      code:c
       }
   }
   //display
   fn display(&self){
      println!("name :{} code:{} of type: {}", self.name, self.code, self.level );
   }
}
fn main(){
   // call the static method
   let c1 = Course::my_static_method("Rust".to_string(), "beginner".to_string(), 132);
   c1.display();
}
