//! A library for making and interacting with Minesweeper boards.
//!
//! The front-end implementation is entirely up to the consumer. This
//! library provides nothing for actually *playing* Minesweeper; it
//! only handles the logic for revealing tiles, flagging tiles, etc.
//!
//! # Implementing a Front-End
//!
//! The `board` and `tile` modules contain all the data and helper
//! methods you will need to create a front-end for actually starting
//! and playing a game of Minesweeper. Of particular interest to the
//! programmer:
//!
//! * Each `Tile` has a state that can be queried, so that your
//! program knows how to represent it to the user. For example, a
//! `Tile` that has `TileState::Flagged` will be represented by a '!'
//! when printed with `Display`. Your program, then, can choose to
//! represent that `Tile` with a specific sprite depending on its
//! state.
//!
//! * The `Tiles` of any given `Board` are contained within a
//! one-dimensional `Vec`, and must be accessed as such. For example:
//! the very first tile (top-left corner) would be at 0. Convenience
//! methods are provided so that you can access a `Tile` with an (x,
//! y) coordinate pair, and vice-versa.

pub mod board;
pub mod tile;
