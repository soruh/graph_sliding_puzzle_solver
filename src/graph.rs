/// Respresents a single block by it's position and size.
/// Also provides helper methods for operations using blocks
#[derive(serde::Serialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Block {
    pub position: (u32, u32),
    pub size: (u32, u32),
}

impl Block {
    pub fn new(position: (u32, u32), size: (u32, u32)) -> Self {
        Self { position, size }
    }

    /// Finds the distance from `self` to other using the taxi-cab metric
    pub fn distance_from(&self, target: (u32, u32)) -> u32 {
        let x_size = (target.0 as i32 - self.position.0 as i32).abs() as u32;
        let y_size = (target.1 as i32 - self.position.1 as i32).abs() as u32;

        x_size + y_size
    }

    /// Checks if `self` overlaps with other
    pub fn overlaps_with(&self, other: &Block) -> bool {
        self.position.0 <= other.position.0 + other.size.0 - 1
            && other.position.0 <= self.position.0 + self.size.0 - 1
            && self.position.1 <= other.position.1 + other.size.1 - 1
            && other.position.1 <= self.position.1 + self.size.1 - 1
    }
}

/// Respresents a board state; It is a node in the graph.
#[derive(serde::Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Board {
    pub size: (u32, u32),
    pub blocks: Vec<Block>,
}

impl Board {
    pub fn new(size: (u32, u32), blocks: Vec<Block>) -> Self {
        Self { size, blocks }
    }

    /// creates an iterator that iterates over all Board states that can be
    /// directly reached from this one
    pub fn neighbors(&self) -> Neighbors {
        Neighbors { board: self.clone(), block: 0, direction: Direction::new() }
    }

    /// try to move a block in a direction.
    /// returns an `Ok` with the resulting board state if successfull and an
    /// `Err` if unseccessfull
    pub fn try_move(&self, block_index: usize, direction: &Direction) -> Result<Board, ()> {
        let block = &self.blocks[block_index];
        let delta = direction.delta();

        let new_x = block.position.0 as i32 + delta.0;
        if new_x < 0 || new_x + block.size.0 as i32 > self.size.0 as i32 {
            // The move is invalid, because it would move
            // out of bounds
            return Err(());
        }

        let new_y = block.position.1 as i32 + delta.1;
        if new_y < 0 || new_y + block.size.1 as i32 > self.size.1 as i32 {
            // The move is invalid, because it would move
            // out of bounds
            return Err(());
        }

        let mut moved = block.clone();
        moved.position.0 = new_x as u32;
        moved.position.1 = new_y as u32;

        for (index, block) in self.blocks.iter().enumerate() {
            if block_index != index {
                if moved.overlaps_with(&block) {
                    // The move is invalid, because it would move
                    // into a different block
                    return Err(());
                }
            }
        }

        // The move is valid; return it
        let mut new_board = self.clone();
        new_board.blocks[block_index] = moved;
        Ok(new_board)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = vec![vec![None; self.size.0 as usize]; self.size.1 as usize];

        let n_digits = (self.blocks.len() as f32).log10().ceil() as usize;

        for (index, block) in self.blocks.iter().enumerate() {
            let position = block.position;
            let size = block.size;

            for i in position.0..position.0 + size.0 as u32 {
                for j in position.1..position.1 + size.1 as u32 {
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
