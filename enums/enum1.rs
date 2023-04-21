fn main() {
    let dir=Direction::up;

    match dir {
        Direction::up =>println!("go up"),
        Direction::down =>println!("go down"),
        Direction::left =>println!("go left"),
        Direction::right =>println!("go right"),
    }
}
enum Direction{
    up,
    down,
    left,
    right,
}