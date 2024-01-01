fn main(){
  
   let course = "Rust".to_string();
   let _course_type = "beginner course".to_string();
   // default format macro 
   let result = format!("{} {}", course, _course_type);
   // passing value in the placeholder in the format macro 
   let result = format!("{1} {0}", course,_course_type);
   println!("{}", result);
}
