fn main(){
    let index = find_first_a(String::from("zohaib"));

    match index{
        Some(index) => println!("The index of the first a is {}", index),
        None => println!("There is no a in the string"),
    }
}

fn find_first_a(s:String) -> Option<i8>{
    for (index,char) in s.chars().enumerate(){
        if char == 'a'{
            return Some (index as i8);
        }
    }

    return None;
}