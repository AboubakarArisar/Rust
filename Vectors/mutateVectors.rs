fn main() {
   // define a vector of size 5
   let mut my_vec = vec![1, 2, 3, 4, 5];
   println!("Initial Vector : {:?}", my_vec);
   for x in my_vec.iter_mut(){
       *x *= 3;
   }
   // print the updated vector
   println!("Updated Vector : {:?}", my_vec);
}
