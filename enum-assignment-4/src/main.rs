#[derive(Debug)]
enum Direction {
    Forward,
    Left,
    Backward,
    Right,
}

fn main() {
    let mut direction = Direction::Forward;
    //Method: 1 -> Without asterisk
    for i in 0..10 {
        direction = direction.next();
        println!("{:#?}", direction.next());
    }
    //Method:2 -> With asterisk
    //direction.move_and_print();
}
// With Asterisk
impl Direction {
    fn move_and_print(&mut self) { 
        for i in 0..10 {
            println!("{:#?}", self);
            match self {
                Direction::Forward => *self = Direction::Left,
                Direction::Left => *self = Direction::Backward,
                Direction::Backward => *self = Direction::Right,
                Direction::Right => *self = Direction::Forward,
            }
        }
    }
} 
// Without asterisk
impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Forward => return Direction::Left,
            Direction::Left => return Direction::Backward,
            Direction::Backward => return Direction::Right,
            Direction::Right => return Direction::Forward,
        }
    }
}