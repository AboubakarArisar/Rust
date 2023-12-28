//finding first vowel problem

fn main() {
    let text = "this is a random text";
    
    if finder(text) == false {
        println!("No any vowel in text")
    }
    else {
        println!("Hurray");
    }
}

fn finder(text:&str)->bool {
    
    for i in text.chars() {
        if i == 'a' || i == 'e' || i == 'i' ||i ==  'o' ||i == 'u' {
            println!("vowel {} is found", i);
            return true;
        }      
    }
    return false;
}
