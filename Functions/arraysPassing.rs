fn main() {
   let arr = [1, 2, 3, 4, 5];
   modify_my_array(arr);
   println!("Array in Driver Function : {:?}", arr);
}
fn modify_my_array(mut arr:[i32;5]){
   arr[2] = 8;
   arr[3] = 9;
   println!("Array in my Function : {:?}", arr);
}
