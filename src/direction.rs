#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn mirror(&self) -> Self {
        // The function mirror will return the new direction when the direction is reflected by
        // the mirror \.
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            Direction::South => Direction::East,
            Direction::West => Direction::North,
        }
    }

    pub fn left(&self) -> Self {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn right(&self) -> Self {
        self.left().mirror()
    }
}
