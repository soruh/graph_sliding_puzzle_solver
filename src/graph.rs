/// Respresents a single block by it's position and size.
/// Also provides helper methods for operations using blocks
#[derive(serde::Serialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Block {
    pub position: (i32, i32),
    pub size: (i32, i32),
}

use std::convert::TryInto;

impl Block {
    pub fn new(position: (u32, u32), size: (u32, u32)) -> Self {
        Self {
            position: (
                position.0.try_into().expect("the supplied position is too big"),
                position.1.try_into().expect("the supplied position is too big"),
            ),
            size: (
                size.0.try_into().expect("the supplied size is too big"),
                size.1.try_into().expect("the supplied size is too big"),
            ),
        }
    }

    /// Finds the distance from `self` to other using the taxi-cab metric
    pub fn distance_from(&self, target: (i32, i32)) -> u32 {
        let x_size = (target.0 - self.position.0).abs() as u32;
        let y_size = (target.1 - self.position.1).abs() as u32;

        x_size + y_size
    }

    /// Checks if `self` overlaps with other
    pub fn overlaps_with(&self, other: &Block) -> bool {
        self.position.0 < other.position.0 + other.size.0
            && other.position.0 < self.position.0 + self.size.0
            && self.position.1 < other.position.1 + other.size.1
            && other.position.1 < self.position.1 + self.size.1
    }
}

/// Respresents a board state; It is a node in the graph.
#[derive(serde::Serialize, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Board {
    pub size: (i32, i32),
    pub blocks: Vec<Block>,
}

impl Board {
    pub fn new(size: (u32, u32), blocks: Vec<Block>) -> Self {
        Self {
            size: (
                size.0.try_into().expect("the supplied size is too big"),
                size.1.try_into().expect("the supplied size is too big"),
            ),
            blocks,
        }
    }

    /// creates an iterator that iterates over all Board states that can be
    /// directly reached from this one
    pub fn neighbors(&self) -> Neighbors {
        Neighbors { board: self.clone(), ..Neighbors::default() }
    }

    /// try to move a block in a direction.
    /// returns an `Ok` with the resulting board state if successfull and an
    /// `Err` if unseccessfull
    pub fn try_move(&self, block_index: usize, direction: &Direction) -> Result<Board, ()> {
        let block = &self.blocks[block_index];
        let delta = direction.delta();

        // Check if the move is invalid, because it would move out of bounds
        let new_x = block.position.0 + delta.0;
        if new_x < 0 || new_x + block.size.0 > self.size.0 {
            return Err(());
        }

        let new_y = block.position.1 + delta.1;
        if new_y < 0 || new_y + block.size.1 > self.size.1 {
            return Err(());
        }

        let mut moved_block = *block; // Copy the block
        moved_block.position.0 = new_x;
        moved_block.position.1 = new_y;

        // Check if the move is invalid, because it would move into a different block
        for (index, block) in self.blocks.iter().enumerate() {
            if block_index != index && moved_block.overlaps_with(&block) {
                return Err(());
            }
        }

        // The move is valid; return it's result
        let mut new_board = self.clone();
        new_board.blocks[block_index] = moved_block;
        Ok(new_board)
    }
}

impl std::fmt::Display for Board {
    #[allow(clippy::cast_sign_loss)] // We know that our values are above 0 since they can only be constructed from unsigned integers
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = vec![vec![None; self.size.0 as usize]; self.size.1 as usize];

        #[allow(clippy::cast_possible_truncation)]
        // we want to truncate the index

        // To overflow a f64s mantisa we would need (2**52) - 1 Blocks,
        // which would require ((2**52) - 1) * 16 ~= 72 PetaBytes of Memory
        // at which point we would long be OOM
        #[allow(clippy::cast_precision_loss)]
        let n_digits = (self.blocks.len() as f64).log10().ceil() as usize;

        for (index, block) in self.blocks.iter().enumerate() {
            let position = block.position;
            let size = block.size;

            for i in position.0..position.0 + size.0 {
                for j in position.1..position.1 + size.1 {
                    grid[j as usize][i as usize] = Some(index);
                }
            }
        }

        let mut res = String::with_capacity((grid[0].len() + 2) * (grid.len() + 2));

        let seperator = "-".repeat(grid[0].len() * (n_digits + 1) + 1);

        res.push_str(&seperator);
        res.push('\n');
        for line in grid {
            res.push('|');

            for number in line {
                if let Some(number) = number {
                    res.push_str(&format!("{:width$}", number, width = n_digits));
                } else {
                    res.push_str(&" ".repeat(n_digits));
                }
                res.push('|');
            }
            res.push('\n');
        }
        res.push_str(&seperator);

        write!(f, "{}", res)
    }
}

/// represents a direction in which a move can occur
/// and provied methods to facilitate moving in that dirrection
/// and moving on to the next direction
#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::new()
    }
}

impl Direction {
    pub fn new() -> Self {
        Self::Up
    }

    /// moves on to the next direction and returns
    /// true if the direction cycled back to it's
    /// the initial value
    pub fn next(&mut self) -> bool {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };

        // Have we gone full circle?
        *self == Self::Up
    }

    /// returns the amount by which a block must be moved
    /// in the x and y directions to move into the direction
    /// specified by `self`
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

/// An Iterator that iterates over all valid board states
/// that can be reached from the current state
#[derive(Default)]
pub struct Neighbors {
    board: Board,
    block: usize,
    direction: Direction,
}

impl Iterator for Neighbors {
    type Item = Board;

    /// Iterates over all blocks in the board, tries to move
    /// each of them in each direction by one and yields all
    /// valid resulting board states
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.direction.next() {
                // advance to next block, if we cycled through directions
                self.block += 1;

                if self.block >= self.board.blocks.len() {
                    // There are no new blocks to check for moves; We are done
                    return None;
                }
            }

            if let Ok(board) = self.board.try_move(self.block, &self.direction) {
                // Found a valid move; Yield it.
                return Some(board);
            }
        }
    }
}
