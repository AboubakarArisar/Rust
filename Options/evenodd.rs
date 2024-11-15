enum Even {
    Some(bool),
    None,
}

fn main() {
    let even = is_even(5);
    match even {
        Even::Some(even) => println!("The number is even"),
        Even::None => println!("The number is odd"),
    }
}

fn is_even(num: i32) -> Even {
    if num % 2 == 0 {
        Even::Some(true)
    } else {
        Even::None
    }
}
