struct Rect{
    width: u32,
    height: u32,
}

impl Rect{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn permimeter(&self) -> u32{
        2 * (self.width + self.height)
    }
}

fn main(){
    let rect = Rect{
        width: 30,
        height: 50,
    };

    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.permimeter());
}