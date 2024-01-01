fn main() {
   // define a vector of size 5
   let mut my_vec = vec![1, 2, 3, 4, 5];
   // print vector
   println!("Vector : {:?}", my_vec);
   // print the capacity of vector
   println!("Capacity of vector: {}", my_vec.capacity());
   // print the length of vector
   println!("Length of the vector : {}",my_vec.len());
   my_vec.push(6);
   my_vec.push(8); 
   // print vector
   println!("Vector : {:?}",my_vec);
   // print the capacity of vector
   println!("Capacity of vector: {}", my_vec.capacity());
   // print the length of vector
   println!("Length of the vector : {}", my_vec.len());
}
