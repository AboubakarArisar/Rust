enum Shape{
    Rectangle,
    Circle
}

fn main(){
let current_shape = Shape::Rectangle;
print_area(current_shape);
}

fn print_area(shape:Shape){
    println!("printing the area of the shape ");
}

