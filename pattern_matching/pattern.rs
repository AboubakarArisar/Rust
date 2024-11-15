enum Department{
    CS,
    BBA,
    AF,
    AI,
    EE
}

fn main(){
    let current_department = Department::CS;
    print_department(current_department);
}


fn print_department(dep:Department){
    match dep{
        Department::CS => println!("Computer Science"),
        Department::BBA => println!("Business Administration"),
        Department::AF => println!("Accounting and Finance"),
        Department::AI => println!("Artificial Intelligence"),
        Department::EE => println!("Electrical Engineering"),
    }
}