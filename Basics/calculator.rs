fn test(a: i32, operator: char ,b: i32) {
    match operator {
            '+' => {
                println!("{}", a + b);
            },
            '-' => {
                println!("{}", a - b);
            },
            '*' => {
                println!("{}", a * b);
            },
            '/' => {
                if b == 0{
                    println!("Division by 0 is undefined");
                }
                else {
                    println!("{}", a / b);
                }
            },
            '%' => {
                if b == 0{
                    println!("Mod 0 is undefined");
                }
                else {
                    println!("{}", a % b);
                }
            },
            _ => println!("{}", "invalid operator"),
        }
}
fn main(){
    print!("3 + 2: ");
    test(3,'+',2);
    print!("3 - 2: ");
    test(3,'-',2);
    print!("3 * 2: ");
    test(3,'*',2);
    print!("3 / 2: ");
    test(3,'/',2);
    print!("3 % 2: ");
    test(3,'%',2);
    print!("3 ( 2: ");
    test(3,'(',2);
    print!("3 ( 0: ");
    test(3, '/', 0)
}
