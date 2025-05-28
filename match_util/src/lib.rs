pub enum Direction {
    East,
    West,
    North,
    South,
}

pub fn direction_match(direction: Direction) {
    match direction {
        Direction::East => println!("East"),
        Direction::West => println!("West"),
        Direction::North | Direction::South => println!("North or South"),
    };
}
