struct User{
    first_name: String,
    last_name: String,
    age: u8,
}
fn main(){
    let user1 = User{
        first_name :String::from("Aboubakar"),
        last_name :String::from("Arisar"),
        age: 20,
    };
    println!("First name: {}", user1.first_name);
    println!("Last name: {}", user1.last_name);
    println!("Age: {}", user1.age);


}