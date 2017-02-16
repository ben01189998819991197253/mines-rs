extern crate mines;

use mines::board::Board;

fn main() {
    let b: Board = Default::default();
    let result = b.reveal_tile(6);
    println!("{:?}", result);

    println!("{:?}", b);

    println!("{}", b);
}
