struct Rectangle<T> {
   width:T,
   height:T
}
fn main() {
   //generic type of i32
   let r1:Rectangle<i32> = Rectangle{width:250, height:150};
   println!("Width:{}, Height:{}", r1.width, r1.height);
   //generic type of String
   let r2:Rectangle<f32> = Rectangle{width:240.0, height:250.0};
   println!("Width:{}, Height:{}", r2.width, r2.height);
   
}
