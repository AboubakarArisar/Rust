fn main() {
    println!("====Password Validation=====");

    let password = "Arisar34@";
    if validator(password) {
        println!("Password is valid");
    } else {
        println!("Password is invalid");
    }
}

fn textChecker(password: &str) -> bool {
    if password.len() >= 8 {
        for i in password.chars() {
            if (i >= 'a' && i <= 'z') || (i >= 'A' && i <= 'Z') {
                return true;
            }
        }
    }
    false
}

fn numberChecker(password: &str) -> bool {
    if password.len() >= 8 {
        for i in password.chars() {
            if i >= '0' && i <= '9' {
                return true;
            }
        }
    }
    false
}

fn validator(password: &str) -> bool {
    let res1 = textChecker(password);
    let res2 = numberChecker(password);
    res1 == res2
}
