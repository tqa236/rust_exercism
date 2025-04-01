#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (rank1, file1) = (self.position.rank, self.position.file);
        let (rank2, file2) = (other.position.rank, other.position.file);

        rank1 == rank2 || file1 == file2 || (rank1 - rank2).abs() == (file1 - file2).abs()
    }
}
