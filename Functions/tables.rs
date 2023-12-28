fn main() {
    let table = 5;
    table_generator(table);
}
fn table_generator(n:i32){
    if n > 0 {
        for i in 1..=10 {
            println!("{} x {} = {} " , i , n , i * n);
        }
    }
}


