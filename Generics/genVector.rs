fn main(){
   let mut my_int_vector: Vec<i32> = vec![1,2];
   my_int_vector.push(3);
   println!("{:?}",my_int_vector);
   // my_int_vector.push("Rust"); // mismatched types error
}
