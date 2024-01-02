
enum KnightMove{
   Horizontal, Vertical
}
fn main() {
   // use enum
   let horizontal_move = KnightMove::Horizontal;
   let vertical_move = KnightMove::Vertical;
   // print the enum values
   println!("Move 1: {:?}", horizontal_move);
   println!("Move 2: {:?}", vertical_move);
}
