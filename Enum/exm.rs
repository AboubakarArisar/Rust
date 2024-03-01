// make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum KnightMove{
   Horizontal(String), Vertical(String)
}
fn main() {
   // invoke an enum
   let horizontal_move = KnightMove::Horizontal("Left".to_string());
   let vertical_move = KnightMove::Vertical("Down".to_string());
   // print enum
   println!("Move 1: {:?}", horizontal_move);
   println!("Movw 2: {:?}", vertical_move);
}
