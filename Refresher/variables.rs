//data types and variables in the rust 

//firsly variables in rust are immutable by default we have to make the mutable if we wanna change it


//integer data type : two types signed and unsigned i and u

//variables which are unused are supposed to be marked with _ in the beginning of the variable name for example _y
fn main() {

let x: i32 = 10; //signed integer
let _y: u32 = 10; //unsigned integer

//floating point data type : f32 and f64

let _z: f32 = 1.023;
let _a: f64 = 10.02354;

//boolean data type : bool

let _b: bool = true;

//character data type : char

let _c: char = 'a';
let _spl_character = '@';


//string data type : str

let _s: &str = "hello world";

    println!("The value of integer x is: {}", x);
}

