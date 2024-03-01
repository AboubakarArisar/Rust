fn main() {
   println!("{:?}", learn_lang("Rust"));
   println!("{:?}", learn_lang("Python"));
}
fn learn_lang(my_lang:&str)-> Option<bool> {
   if my_lang == "Rust" {
      Some(true)
   } else {
      None
   }
}
