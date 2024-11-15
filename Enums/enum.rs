enum Direction{
    North,
    South,
    West,
    East
}

fn main(){
    let current_direction = Direction::North;
    move_direction(current_direction);
}

fn move_direction(direction: Direction){
    match direction{
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::West => println!("Moving West"),
        Direction::East => println!("Moving East"),
    }
}