#![warn(missing_docs)]
/*! [Re-exported] Data structures and methods for creating and
maniuplating Minesweeper boards.

# Examples

Instantiating a new `Board`:

```
use mines::Board;

// A default 8x8 board
let b: Board = Default::default();
// Reveal the 8th tile using a linear index
let result = b.reveal_tile(7);
// Ensure the operation was a success
assert_eq!(Ok(()), result);
// Get the index of the tile in the bottom-right corner
let index = b.linear_coords((7, 7));
b.reveal_tile(index);
// Print what the user should see to stdout
println!("{}", b);
```

Example output:

```text
?????1.. <-- The revealed tile
?????1..
?????21.
??????21
????????
????????
????????
???????* <-- Other revealed tile
```

Debug-printing a custom-sized `Board`:

```
# use mines::Board;
// A 9x9 board with 20 mines
let b: Board = Board::new(9, 9, 20);
// The board will NOT be generated until
// an initial tile is revealed
b.reveal_tile(0);
// See the debug output
println!("{:?}", b);
```

Example output:

```text
.1*23*311
.12*4*3*2
.135*322*
.1***2.11
.12322121
111111*3*
*12*3214*
112**223*
..123*2*2
```
*/


use std::cell::{Cell, RefCell};
use std::default::Default;
use std::fmt;
use std::collections::HashMap;

use self::rand::Rng;

use tile::{Tile, TileState};

extern crate rand;

/// Representation of a standard Minesweeper board.
#[derive(Clone)]
pub struct Board {
    /// The total number of bombs (revealed or not) on the
    /// `Board`.
    pub num_mines: usize,
    /// Keeps track of whether the `Board` has been generated.
    was_generated: Cell<bool>,
    /// The horizontal width.
    pub width: usize,
    /// The vertical height.
    pub height: usize,
    /// Collection of `Tiles` that make up the `Board`.
    pub tiles: Vec<RefCell<Tile>>,
}

impl Default for Board {
    fn default() -> Board {
        const SIZE: usize = 8;

        Board {
            num_mines: 10,
            was_generated: Cell::new(false),
            width: SIZE,
            height: SIZE,
            tiles: vec![RefCell::new(Tile::default()); SIZE * SIZE],
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();

        for (i, tile_ref) in self.tiles.iter().enumerate() {
            s.push_str(&format!("{:?}", *tile_ref.borrow()));
            if (i + 1) % self.width == 0 {
                s.push_str("\n");
            }
        }

        write!(f, "{}", s)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();

        for (i, tile_ref) in self.tiles.iter().enumerate() {
            s.push_str(&format!("{}", *tile_ref.borrow()));
            if (i + 1) % self.width == 0 {
                s.push_str("\n");
            }
        }

        write!(f, "{}", s)
    }
}

impl Board {
    /// Creates a new `Board`.
    ///
    /// # Panics
    ///
    /// This function will panic if the programmer attempts to make a
    /// `Board` smaller than 3x3, or if there are too many mines to
    /// make a functioning `Board`. There must be fewer than (`width`
    /// * `height` - 9) mines.
    pub fn new(width: usize, height: usize, num_mines: usize) -> Board {
        if width * height <= 9 {
            panic!("Tried to make too small of a board!");
        }
        if num_mines >= (width * height) - 9 {
            panic!("Too many mines to make a functioning board! Mines passed: {}, Maximum mines: \
                    {}",
                   num_mines,
                   (width * height) - 10);
        }

        Board {
            num_mines: num_mines,
            was_generated: Cell::new(false),
            width: width,
            height: height,
            tiles: vec![RefCell::new(Tile::default()); width * height],
        }
    }

    /// Returns the indices of any adjacent tiles.
    ///
    /// `Board` represents its grid of tiles as a one-dimensional
    /// `Vec<Tile>`. Given the `index` of a `Tile`, this function will
    /// return an `Vec<usize>` of the indices of all the tiles
    /// surrounding it.
    ///
    /// # Examples
    ///
    /// Finding the indices of the `Tiles` surrounding the first
    /// `Tile`:
    ///
    /// ```
    /// use mines::board::Board;
    ///
    /// // An 8x8 Board
    /// let b: Board = Default::default();
    /// let indices: Vec<usize> = b.adjacent_tile_indices(0);
    /// assert_eq!(indices, vec![1, 8, 9]);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the programmer passes an `index`
    /// that is not within the bounds of the grid, or if the
    /// dimensions of the `Board` are not at least 3x3.
    pub fn adjacent_tile_indices(&self, index: usize) -> Vec<usize> {
        adjacent_indices(index, self.width, self.tiles.len())
    }

    /// Flood-reveals any available `Tiles`, allowing the user to see
    /// their values. Returns a `Result` indicating whether the
    /// reveals were successful.
    ///
    /// The first time this function is called, the `Board` will place
    /// its bombs and generate values for its `Tiles`.
    ///
    /// # Errors
    ///
    /// This function will return an error if any `Tile` was not in a
    /// revealable `TileState`, such as if it was already revealed. It
    /// is safe to discard this error; it is only for the programmer.
    pub fn reveal_tile(&self, index: usize) -> Result<(), &'static str> {
        if !self.was_generated.get() {
            self.generate(index);
        }
        // Then flood-fill reveal, starting with the tile at index.
        let result = self.tiles[index].borrow_mut().reveal();
        if result.is_err() {
            result
        } else {
            self.flood_reveal(index)
        }
    }


    /// Toggles this `Tile` as flagged. If it is flagged, the user
    /// will not be able to reveal it (and uncover a bomb). Returns a
    /// `Result` indicating whether the flag was successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Board` has not been
    /// generated yet, or if the `Tile` was not in a flaggable
    /// `TileState` (such as if it is already revealed.) It is safe to
    /// discard this error; it is only for the programmer.
    pub fn flag_tile(&self, index: usize) -> Result<(), &'static str> {
        if !self.was_generated.get() {
            // NOTE: gnome-mines allows pre-generation flagging, it
            // just removes the ones it encounters during the flood fill
            return Err("Cannot flag Tile: The Board has not been generated yet.");
        }
        self.tiles[index].borrow_mut().flag()
    }

    fn generate(&self, index: usize) {
        self.was_generated.set(true);

        // We must not put a bomb on the adjacent 8 tiles
        let mut invalid_locations = self.adjacent_tile_indices(index);
        // Nor the original tile
        invalid_locations.push(index);
        let invalid_locations = invalid_locations;

        // Tile cannot be in an invalid location or already a bomb
        let is_valid = |x: usize| {
            if self.tiles[x].borrow().is_bomb {
                return false;
            }

            for index in &invalid_locations {
                if x == *index {
                    return false;
                }
            }

            true
        };

        for _ in 0..self.num_mines {
            loop {
                let i = rand::thread_rng().gen_range(0, self.tiles.len());
                if is_valid(i) {
                    let mut tile = self.tiles[i].borrow_mut();
                    tile.is_bomb = true;
                    break;
                }
            }
        }

        // Add tile values
        for (index, tile_ref) in self.tiles.iter().enumerate() {
            if tile_ref.borrow().is_bomb {
                continue;
            }

            let indices = self.adjacent_tile_indices(index);
            let mut num_bombs: usize = 0;
            for i in &indices {
                if self.tiles[*i].borrow().is_bomb {
                    num_bombs += 1;
                }
            }

            tile_ref.borrow_mut().adjacent_bombs = num_bombs;
        }
    }

    fn flood_reveal(&self, index: usize) -> Result<(), &'static str> {
        let mut result: Result<(), &'static str> = Ok(());

        // We use HashMap so that we do not have any duplicated values
        // in our todo list
        let mut current: HashMap<usize, usize> = HashMap::new();
        current.insert(index, index);

        'outer: while !current.is_empty() {
            let mut todo: HashMap<usize, usize> = HashMap::new();
            for index in current.values() {
                // Reveal the tile, quitting if there's an Err
                {
                    let mut tile = self.tiles[*index].borrow_mut();
                    let reveal_result = tile.reveal();
                    if reveal_result.is_err() {
                        result = reveal_result;
                        break 'outer;
                    }
                }

                // Then add any revealable tiles if they're not
                // already in the todo list
                let surrounding = self.adjacent_tile_indices(*index);
                for i in &surrounding {
                    if self.tile_should_auto_reveal(*i) {
                        todo.entry(*i).or_insert(*i);
                    }
                }
            }
            current = todo;
        }

        result
    }

    fn tile_should_auto_reveal(&self, index: usize) -> bool {
        // A tile should be revealed by the flood_reveal method if it
        // has not already been revealed, and if it is adjacent to an
        // empty tile that has been revealed.
        match self.tiles[index].borrow().state {
            TileState::Revealed => false,
            _ => self.tile_touches_revealed_empty(index),
        }
    }

    fn tile_touches_revealed_empty(&self, index: usize) -> bool {
        let indices = self.adjacent_tile_indices(index);
        let mut touches_empty = false;

        for index in indices {
            let tile = self.tiles[index].borrow();
            if tile.adjacent_bombs == 0 {
                // match tile.state {
                //     TileState::Revealed => {
                //         touches_empty = true;
                //         break;

                //     }
                //     _ => {}
                // }
                if let TileState::Revealed = tile.state {
                    touches_empty = true;
                    break;
                }
            }
        }

        touches_empty
    }

    /// Converts an (x, y) coordinate pair to a 1D index.
    ///
    /// `Board` represents its grid of tiles as a one-dimensional
    /// `Vec<Tile>`. Given a coordinate pair (x, y) referring to the
    /// location of `Tile`, this function will return the 1D index
    /// that corresponds to that `Tile`.
    ///
    /// # Examples
    ///
    /// Finding the index from coordinates of the `Tile` at (3, 4):
    ///
    /// ```
    /// use mines::board::Board;
    ///
    /// // An 8x8 Board
    /// let b: Board = Default::default();
    /// let tile_index = b.linear_coords((3, 4));
    /// assert_eq!(tile_index, 35);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the programmer passes a coordinate
    /// pair that is not within the bounds of the grid.
    pub fn linear_coords(&self, p: (usize, usize)) -> usize {
        if p.0 >= self.width || p.1 >= self.height {
            panic!("Tried to get the index of a Tile that wasn't within the bounds of the grid! \
                    Coordinates passed: ({}, {}), Grid bounds: {}x{}",
                   p.0,
                   p.1,
                   self.width,
                   self.height);
        }
        linear_coords(p, self.width)
    }

    /// Converts a 1D index to an (x, y) coordinate pair.
    ///
    /// `Board` represents its grid of tiles as a one-dimensional
    /// `Vec<Tile>`. Given the `index` of a `Tile`, this function will
    /// return an (x, y) coordinate pair (zero indexed) that refers to
    /// that `Tile`.
    ///
    /// # Examples
    ///
    /// Finding the coordinates of the first `Tile`:
    ///
    /// ```
    /// use mines::board::Board;
    ///
    /// // An 8x8 Board
    /// let b: Board = Default::default();
    /// let first_tile_coords = b.cartesian_coords(0);
    /// assert_eq!(first_tile_coords, (0, 0));
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the programmer passes an `index`
    /// that is not within the bounds of the grid.
    pub fn cartesian_coords(&self, index: usize) -> (usize, usize) {
        if index >= self.tiles.len() {
            panic!("Tried to get the coordinates of a Tile that wasn't within the bounds of the \
                    grid! Index passed: {}, Grid length: {}",
                   index,
                   self.tiles.len());
        }
        cartesian_coords(index, self.width)
    }
}

fn linear_coords(p: (usize, usize), width: usize) -> usize {
    let (x, y) = p;
    (width * y) + x
}

fn cartesian_coords(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

fn adjacent_indices(index: usize, width: usize, length: usize) -> Vec<usize> {
    // In an actual array, Rust will enforce whether the index is out
    // of bounds.
    if index >= length {
        panic!("Tried to find adjacent indices using an index greater than the length of the \
                grid! Passed index: {}, Grid length: {}",
               index,
               length);
    }

    let mut indices: Vec<usize> = Default::default();

    // NOTE: This will behave badly with grids of length < 9
    // (although WHY would you even do that?)
    // Definitely do not use with grids that are not at least 3x3
    if length < 9 {
        panic!("Tried to find adjacent indices in a grid that is not at least 3x3! Grid length: \
                {}",
               length);
    }

    // Lots of literal edge cases to detect.
    // The corners must be checked first, THEN the sides.
    // HOW TO DERIVE THE MAGIC NUMBERS:
    // 1. Make an arbitrary grid, I did a 5x4 one.
    // 2. Number the tiles sequentially, [0, length)
    // 3. For each corner or edge, sketch out what the 3x3 section would look like
    //    if it were a whole piece (it's not, because it's on an edge/corner)
    // 4. Figure out the relationship between the index and the indices of the
    //    expected/desired tiles that are surrounding it. Do not include tiles that
    //    are not actually part of the grid.
    match index {
        0 => {
            // top-left corner
            indices.push(index + 1);
            indices.push(index + width);
            indices.push(index + width + 1);
        }
        index if index == (width - 1) => {
            // top-right corner
            indices.push(width - 2);
            indices.push(2 * width - 2);
            indices.push(2 * width - 1);
        }
        index if index == (length - width) => {
            // bottom-left corner
            indices.push((length - width) - width);
            indices.push((length - width) - width + 1);
            indices.push(index + 1);
        }
        index if index == (length - 1) => {
            // bottom-right corner
            indices.push(index - width - 1);
            indices.push(index - width);
            indices.push(index - 1);
        }
        index if index % width == 0 => {
            // left side
            indices.push(index - width);
            indices.push(index - width + 1);
            indices.push(index + 1);
            indices.push(index + width);
            indices.push(index + width + 1);
        }
        index if (index + 1) % width == 0 => {
            // right side
            indices.push(index - width - 1);
            indices.push(index - width);
            indices.push(index - 1);
            indices.push(index + width - 1);
            indices.push(index + width);
        }
        _ => {
            // Assume it's a well-behaved index
            // Attempt to push all 8 surrounding indices
            let index = index as i32;
            let width = width as i32;
            let length = length as i32;

            let is_valid = |x: i32| x >= 0 && x < length;

            let surrounding_eight = [(index - width - 1),
                                     (index - width),
                                     (index - width + 1),
                                     (index - 1),
                                     (index + 1),
                                     (index + width - 1),
                                     (index + width),
                                     (index + width + 1)];
            for i in &surrounding_eight {
                if is_valid(*i) {
                    indices.push(*i as usize);
                }
            }
        }
    }

    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_coords() {
        const WIDTH: usize = 5;

        struct Test {
            p: (usize, usize),
            expected: usize,
        };

        let tests = [Test {
                         p: (2, 1),
                         expected: 7,
                     },
                     Test {
                         p: (4, 0),
                         expected: 4,
                     },
                     Test {
                         p: (0, 2),
                         expected: 10,
                     }];

        for test in &tests {
            assert_eq!(test.expected, linear_coords(test.p, WIDTH))
        }
    }

    #[test]
    fn test_cartesian_coords() {
        const WIDTH: usize = 5;

        struct Test {
            index: usize,
            expected: (usize, usize),
        }

        let tests = [Test {
                         index: 4,
                         expected: (4, 0),
                     },
                     Test {
                         index: 6,
                         expected: (1, 1),
                     },
                     Test {
                         index: 10,
                         expected: (0, 2),
                     }];

        for test in &tests {
            assert_eq!(test.expected, cartesian_coords(test.index, WIDTH))
        }
    }

    #[test]
    fn test_adjacent_indices() {
        // Mock grid of 5x4
        const LENGTH: usize = 20;
        const WIDTH: usize = 5;

        struct Test {
            index: usize,
            expected: Vec<usize>,
        }

        let tests = [Test {
                         // top-left
                         index: 0,
                         expected: vec![1, 5, 6],
                     },
                     Test {
                         // top-middle
                         index: 2,
                         expected: vec![1, 3, 6, 7, 8],
                     },
                     Test {
                         // top-right
                         index: 4,
                         expected: vec![3, 8, 9],
                     },
                     Test {
                         // left
                         index: 10,
                         expected: vec![5, 6, 11, 15, 16],
                     },
                     Test {
                         // middle
                         index: 7,
                         expected: vec![1, 2, 3, 6, 8, 11, 12, 13],
                     },
                     Test {
                         // right
                         index: 14,
                         expected: vec![8, 9, 13, 18, 19],
                     },
                     Test {
                         // bottom-left
                         index: 15,
                         expected: vec![10, 11, 16],
                     },
                     Test {
                         // bottom-middle
                         index: 17,
                         expected: vec![11, 12, 13, 16, 18],
                     },
                     Test {
                         // bottom-right
                         index: 19,
                         expected: vec![13, 14, 18],
                     }];

        for test in &tests {
            assert_eq!(adjacent_indices(test.index, WIDTH, LENGTH), test.expected);
        }
    }
}
