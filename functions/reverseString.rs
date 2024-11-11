fn reverseString(s: String) -> String{
    let mut i = 0;
    for (i = 0; i < s.length() / 2; i++) {
        let temp = s[i];
        s[i] = s[s.length() - i - 1];
        s[s.length() - i - 1] = temp;
    }
    return s;
}

fn main() {
    let s = "hello".to_string();
    let result = reverseString(s);
    println!("The reverse of {} is {}", s, result);
}