fn main() {
   println!("{:?}",file_found(true)); // invoke function by passing true 
   println!("{:?}",file_found(false)); // invoke function by passing false
}
fn file_found(i:bool) -> Result<i32,bool> {
   if i { // if true
      Ok(200) // return Ok(200)
   } else { // if false
      Err(false) // return Err(false)
   }
}
