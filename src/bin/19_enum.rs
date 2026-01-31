enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let my_direction = Direction::Up;

    match my_direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Right => println!("Right"),
        Direction::Left => println!("Left"),
    }
}
