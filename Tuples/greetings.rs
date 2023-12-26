fn main() {
    let tuple = ("Aboubakar", 19);
    greetings(tuple);
}

fn greetings(t: (&str, i32)) {
    println!("Hello, {}! your age is {}.", t.0,t.1);
}
