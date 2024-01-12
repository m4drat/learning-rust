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
        (rank >= 0 && file >= 0 && rank.abs() < 8 && file.abs() < 8).then_some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn rank(&self) -> i32 {
        self.position.rank
    }

    pub fn file(&self) -> i32 {
        self.position.file
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // Check if have the same rank or file.
        if self.rank() == other.rank() || self.file() == other.file() {
            return true;
        }

        // Check if are on the same diagonal.
        let rank_diff = (self.rank() - other.rank()).abs();
        let file_diff = (self.file() - other.file()).abs();

        rank_diff == file_diff
    }
}
