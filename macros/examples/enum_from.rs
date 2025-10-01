use macros::EnumFrom;

#[derive(EnumFrom, Debug)]
enum Direction {
    Up,
}

fn main() {
    println!("{:?}", Direction::Up);
}