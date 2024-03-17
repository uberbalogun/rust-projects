enum Direction {

    Up,
    Down,
    Right,
    Left,
}


fn main() {

    let go = Direction::Down;
        
    match go {
        Direction::Up => println!("go up!"),
        Direction::Down => println!("go down!"),
        Direction::Right => println!("go right"),
        Direction::Left => println!("go left"),

    }
}
