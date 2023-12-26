//shadowing

fn main() {
    let outer = 8;
    {
        println!("Before shadowing {}", outer);
        let outer = 9;
        println!("After shadowing {}", outer);
    }
}
