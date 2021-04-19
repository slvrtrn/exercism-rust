#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            position: (x, y),
            direction,
        }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            position: self.position,
            direction: new_direction,
        }
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            position: self.position,
            direction: new_direction,
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.position;
        let new_position = match self.direction {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };
        Self {
            position: new_position,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, c| {
            match c {
                'R' => r.turn_right(),
                'A' => r.advance(),
                'L' => r.turn_left(),
                _ => panic!("Unrecognized command: {}", c)
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
