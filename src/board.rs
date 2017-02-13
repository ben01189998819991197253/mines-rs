#![warn(missing_docs)]

use std::cell::{Cell, RefCell};
use std::default::Default;

use tile::Tile;

extern crate rand;

/// `Board` represents a standard Minesweeper board with `Tiles`.
#[derive(Clone)]
pub struct Board {
    /// The total number of bombs (revealed or not) on the
    /// `Board`.
    pub num_mines: u32,
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

impl Board {
    fn get_surrounding_tiles(&self, index: usize) -> Vec<usize> {
        get_surrounding_tiles(index, self.width, self.tiles.len())
    }
}

fn get_surrounding_tiles(index: usize, width: usize, length: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = Default::default();

    // NOTE: This will behave badly with grids of length <= 9
    // (although WHY would you even do that?)

    // Lots of literal edge cases to detect :(
    // The corners must be checked first, THEN the sides.
    // HOW TO DERIVE THE MAGIC NUMBERS:
    // 1. Make an arbitrary grid, I did a 5x4 one.
    // 2. Number them sequentially, 0..length
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
    fn test_get_surrounding_tiles() {
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
            assert_eq!(get_surrounding_tiles(test.index, WIDTH, LENGTH),
                       test.expected);
        }
    }
}
