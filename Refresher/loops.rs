fn main(){
    //while loop to print the table of any number
    let num = 2;
    let mut counter = 1;
    println!("Table of {} using while loop", num);
    while counter <= 10{
        println!("{} x {} = {}", num, counter, num*counter);
        counter += 1;
    }

    //for loop to print the table of any number
    println!("Table of {} using for loop", num);
    for i in 1..=10{
        println!("{} x {} = {}", num, i, num*i);
    }
}