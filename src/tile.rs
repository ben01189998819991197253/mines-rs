#![warn(missing_docs)]
//! [Re-exported] Methods and data structures for individual tiles on
//! a Minesweeper board.

use std::fmt;
use std::result::Result;
use std::default::Default;

/// Representation of one square on a standard Minesweeper board.
#[derive(Clone)]
pub struct Tile {
    /// Corresponds to what one would see if this `Tile` were
    /// revealed. A value of 2 would indicate the `Tile` is adjacent
    /// to 2 bombs, a value of 0 would mean it isn't surrounded by any
    /// bombs, etc.
    pub adjacent_bombs: usize,
    /// Refers to the current condition of this `Tile`.
    pub state: TileState,
    /// Indicates whether this `Tile` is a bomb.
    pub is_bomb: bool,
}

impl Tile {
    /// Marks this `Tile` as revealed, allowing the user to see its
    /// value. Returns a `Result` indicating whether the reveal was
    /// successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Tile` was not in a
    /// revealable `TileState`, such as if it is already revealed. It
    /// is safe to discard this error; it is only for the programmer.
    pub fn reveal(&mut self) -> Result<(), &'static str> {
        match self.state {
            TileState::Hidden | TileState::Revealed => {
                self.state = TileState::Revealed;
                Ok(())
            }
            _ => Err("Tried to reveal a Tile that can't be revealed!"),
        }
    }

    /// Toggles this `Tile` as flagged. If it is flagged, the user
    /// will not be able to reveal it (and uncover a bomb). Returns a
    /// `Result` indicating whether the flag was successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Tile` was not in a
    /// flaggable `TileState`, such as if it is already revealed. It
    /// is safe to discard this error; it is only for the programmer.
    pub fn flag(&mut self) -> Result<(), &'static str> {
        match self.state {
            TileState::Hidden => {
                self.state = TileState::Flagged;
                Ok(())
            }
            TileState::Flagged => {
                self.state = TileState::Hidden;
                Ok(())
            }
            _ => Err("Tried to flag a Tile that can't be flagged!"),
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            adjacent_bombs: 0,
            state: TileState::Hidden,
            is_bomb: false,
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // "What gets printed to the screen?"
        // In order of priority:
        // 1. Whether it's a bomb
        // 2. Whether it's adjacent to bombs
        // 3. Whether it's not adjacent to bombs
        let adjacent_bombs = self.adjacent_bombs.to_string();

        let s = if self.is_bomb {
            "*"
        } else if adjacent_bombs != "0" {
            adjacent_bombs.as_str()
        } else {
            "."
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // "What gets printed to the screen?"
        // In order of priority:
        // 1. Whether it's Flagged or Hidden
        // (Assuming that it's been Revealed)
        // 2. Whether it's a bomb
        // 3. Whether it's adjacent to bombs
        // 4. Whether it's not adjacent to bombs
        let debug_string = format!("{:?}", self);

        let s = match self.state {
            TileState::Flagged => "!",
            TileState::Hidden => "?",
            TileState::Revealed => debug_string.as_str(),
        };

        write!(f, "{}", s)
    }
}

/// Corresponds to the current condition of a `Tile`.
#[derive(Clone)]
pub enum TileState {
    /// The `Tile` has not been clicked on, and has an unknown value
    /// to the user.
    Hidden,
    /// The `Tile` has been clicked on, and the user can see what is
    /// underneath.
    Revealed,
    /// The user has marked this `Tile` as containing a bomb.
    Flagged,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_print() {
        let mut t: Tile = Default::default();
        assert_eq!(format!("{}", t), "?");

        t.state = TileState::Revealed;
        assert_eq!(format!("{}", t), ".");

        t.adjacent_bombs = 2;
        assert_eq!(format!("{}", t), "2");

        t.is_bomb = true;
        assert_eq!(format!("{}", t), "*");

        t.state = TileState::Flagged;
        assert_eq!(format!("{}", t), "!");
    }

    #[test]
    fn test_debug_print() {
        let mut t: Tile = Default::default();
        assert_eq!(format!("{:?}", t), ".");

        t.adjacent_bombs = 2;
        assert_eq!(format!("{:?}", t), "2");

        t.is_bomb = true;
        assert_eq!(format!("{:?}", t), "*");
    }
}
