enum KnightMove{
   Horizontal,Vertical
}
// print function 
fn print_direction(direction:KnightMove) {
   // match statement
   match direction {
      //execute if knight move is horizontal
      KnightMove::Horizontal => {
         println!("Move in horizontal direction");
      },
       //execute if knight move is vertical
      KnightMove::Vertical => {
         println!("Move in vertical direction");
      }
   }
}
fn main() {
   // invoke function `print_direction`
   let knight1 = KnightMove::Horizontal;
   let knight2 = KnightMove::Vertical;
   print_direction(knight1);
   print_direction(knight2);
}
