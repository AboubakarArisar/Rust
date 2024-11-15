fn main() {
    let str: String = String::from("hello");
    println!("The length of 'hello' is {}", get_length(&str));
}

fn get_length(s: &String) -> usize {
    s.chars().count()
}
