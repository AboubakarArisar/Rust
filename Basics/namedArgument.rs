//named arguments placeholder

fn main() {
    let base: i32;
    let exponent: i32;
    let res: i32;

    println!(
        "{base} power {exponent} is {res}",
        base = 2,
        exponent = 3,
        res = 2 * 2 * 2
    );
}
