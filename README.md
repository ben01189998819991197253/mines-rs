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

## Contributing

...is welcomed! Please submit any and all pull requests or issues.

Running your code through `rustfmt` would be nice.

## License

Dual-licensed under the MIT or Apache v2.0 Licenses, at your
preference. See `LICENSE-MIT` or `LICENSE-APACHE` for details.
