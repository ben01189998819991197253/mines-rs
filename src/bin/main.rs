extern crate mines;

use mines::board::Board;

fn main() {
    let b: Board = Board::new(9, 9, 20);
    b.reveal_tile(0);
    println!("{:?}", b);
}
