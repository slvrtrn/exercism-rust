#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (f1, f2) = (self.position.file, other.position.file);
        let (r1, r2) = (self.position.rank, other.position.rank);
        return if r1 == r2 || f1 == f2 {
            true
        } else {
            i32::abs(r1 - r2) == i32::abs(f1 - f2)
        };
    }
}
