# mines-rs

Work-in-progress library for making Minesweeper boards. Actually
*playing* Minesweeper is dependent on the the program that uses this
crate.

## Building

### Including the crate within another project

Inside your project's `Cargo.toml`:

```toml
# ...
[dependencies]
mines = { git = "https://github.com/ben01189998819991197253/mines-rs.git" }
```

### Building the crate (for development purposes)

```bash
git clone https://github.com/ben01189998819991197253/mines-rs.git
cd mines-rs
cargo test && cargo build
```

## Usage

See the documentation by running:

```bash
cargo doc --no-deps --open
```

in the crate root. It will automatically open in your default web
browser.

### Quickstart

Assuming you've included the crate with your project:

```rust
extern crate mines;

use mines::Board;

fn main() {
   // Default 8x8 grid with 10 mines
   let board = Board::default();

   // Reveal the tile at (0, 4)
   let index = board.linear_coords((0, 4));

   // Revealing a tile returns a Result<_, &'static str> depending on
   // whether it was able to flood-reveal the tiles
   let result = board.reveal_tile(index);
   assert_eq!(Ok(()), result);

   println!("{}", board);
}
```

Example output:

```
????????
12??????
.124????
...2????
...2????
...111??
11...1??
?1...1??
```

## Contributing

...is welcomed! Please submit any and all pull requests or issues.

Running your code through `rustfmt` would be nice.

## License

Dual-licensed under the MIT or Apache v2.0 Licenses, at your
preference. See `LICENSE-MIT` or `LICENSE-APACHE` for details.
