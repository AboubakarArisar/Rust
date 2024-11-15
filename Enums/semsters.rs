enum Semester{
    CS(u8),
    _AF(u8),
    _BED(u8),
}

fn main(){
    let current_semester = Semester::CS(8);
    print_semester(current_semester);
}

fn print_semester(sem:Semester){
    println!("printing the semester ");
}