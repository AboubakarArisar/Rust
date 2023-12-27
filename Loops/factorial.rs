fn test(n:i32) {
   let mut factorial = 1; // define a mutable variable factorial

   if n < 0 { // check if factorial is less than zero
      println!("0"); // print 0
   }
   else if n == 0 {  // check if factorial is equal to 0
      println!("1"); // print 1
   }
   else // go here if the above two conditions are false
   {
      for i in 1..n + 1{
         factorial = factorial * i  
      }
      println!("{}", factorial); // print the factorial 
   }
}
fn main(){
    print!("factorial (4) : ");
    test(4);
    print!("factorial (6) : ");
    test(6);
}
