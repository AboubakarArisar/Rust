fn main() {
   // define a vector of size 5
   let my_vec = vec![1, 2, 3, 4, 5];
   let slice:&[i32] = &my_vec[2..4];
   // print the vector
   println!("Slice of the vector : {:?}",slice);
}
