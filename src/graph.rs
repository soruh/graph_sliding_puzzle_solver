#[derive(serde::Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Block {
    pub position: (u32, u32),
    pub size: (u32, u32),
}

impl Block {
    pub fn new(position: (u32, u32), size: (u32, u32)) -> Self {
        Self { position, size }
    }

    pub fn distance_from(&self, target: (u32, u32)) -> u32 {
        (target.0 as i32 - self.position.0 as i32).abs() as u32
            + (target.1 as i32 - self.position.1 as i32).abs() as u32
    }

    pub fn overlaps_with(&self, other: &Block) -> bool {
        self.position.0 <= other.position.0 + other.size.0 - 1
            && other.position.0 <= self.position.0 + self.size.0 - 1
            && self.position.1 <= other.position.1 + other.size.1 - 1
            && other.position.1 <= self.position.1 + self.size.1 - 1
    }
}

#[derive(serde::Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Board {
    pub size: (u32, u32),
    pub blocks: Vec<Block>,
}

impl Board {
    pub fn new(size: (u32, u32), blocks: Vec<Block>) -> Self {
        Self { size, blocks }
    }

    pub fn neighbors(&self) -> Neighbors {
        Neighbors {
            board: self.clone(),
            block: 0,
            direction: Direction::new(),
        }
    }

    pub fn try_move(&self, block_index: usize, direction: &Direction) -> Result<Board, ()> {
        let block = &self.blocks[block_index];
        let delta = direction.delta();

        let new_x = block.position.0 as i32 + delta.0;
        if new_x < 0 || new_x + block.size.0 as i32 > self.size.0 as i32 {
            // println!("{:?} is out of bounds", block);
            // println!("{} < 0: {}", new_x, new_x < 0);
            // println!(
            //     "{} + {} > {}: {}",
            //     new_x,
            //     block.size.0,
            //     self.size.0,
            //     new_x + block.size.0 as i32 > self.size.0 as i32
            // );
            return Err(());
        }

        let new_y = block.position.1 as i32 + delta.1;
        if new_y < 0 || new_y + block.size.1 as i32 > self.size.1 as i32 {
            // println!("{:?} is out of bounds", block);
            // println!("{} < 0: {}", new_y, new_y < 0);
            // println!(
            //     "{} + {} > {}: {}",
            //     new_y,
            //     block.size.0,
            //     self.size.0,
            //     new_y + block.size.0 as i32 > self.size.0 as i32
            // );
            return Err(());
        }

        let mut moved = block.clone();
        moved.position.0 = new_x as u32;
        moved.position.1 = new_y as u32;

        let mut successful = true;
        for (index, block) in self.blocks.iter().enumerate() {
            if block_index != index {
                if moved.overlaps_with(&block) {
                    // println!("{:?} overlaps with {:?}", moved, block);
                    successful = false;
                    break;
                }
            }
        }

        if successful {
            let mut new_board = self.clone();

            new_board.blocks[block_index] = moved;

            Ok(new_board)
        } else {
            Err(())
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = vec![vec![' '; self.size.0 as usize]; self.size.1 as usize];

        for (index, block) in self.blocks.iter().enumerate() {
            let position = block.position;
            let size = block.size;

            for i in position.0..position.0 + size.0 as u32 {
                for j in position.1..position.1 + size.1 as u32 {
                    grid[j as usize][i as usize] = std::char::from_digit(index as u32, 10).unwrap();
                }
            }
        }

        let mut res = String::with_capacity((grid[0].len() + 2) * (grid.len() + 2));

        let seperator = "-".repeat(grid[0].len() + 2);

        res.push_str(&seperator);
        res.push('\n');
        for line in grid {
            res.push('|');
            for character in line {
                res.push(character);
            }
            res.push('|');
            res.push('\n');
        }
        res.push_str(&seperator);

        write!(f, "{}", res)
    }
}

#[derive(Debug)]
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

    pub fn next(&mut self) -> bool {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };

        if let Self::Up = *self {
            true
        } else {
            false
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

pub struct Neighbors {
    board: Board,
    block: usize,
    direction: Direction,
}

impl Iterator for Neighbors {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Advance to next direction
            if self.direction.next() {
                // advance to next block, if we cycled
                self.block += 1;

                if self.block >= self.board.blocks.len() {
                    // println!("No more possibilites for moves");
                    return None; // We are done
                }
            }

            // println!(
            //     "block: {} => {:?}",
            //     self.block, self.board.blocks[self.block],
            // );
            // println!("direction: {:?}", self.direction);

            if let Ok(board) = self.board.try_move(self.block, &self.direction) {
                // println!("Found a valid move.");
                return Some(board);
            }
        }
    }
}
