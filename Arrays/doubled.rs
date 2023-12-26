fn main() {
  let mut array:[i64;5] = [1,2,3,4,5];
  println!("Original Array : {:?} ", array);
  array[0] *=2;
  array[1] *=2;
  array[2] *=2;
  array[3] *=2;
  array[4] *=2;

  println!("Doubled Array : {:?} ",array);

}
