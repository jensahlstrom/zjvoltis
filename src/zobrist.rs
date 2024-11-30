pub struct Zobrist {
    // 16 pieces on 10 * 10 squares
    pub pieces: [[[u64; 16]; 10]; 10],
    pub white_to_move: u64,
}

impl Zobrist {
    pub fn piece_index(piece: char) -> usize {
        match piece {
            'Z' => 0,
            'J' => 1,
            'V' => 2,
            'O' => 3,
            'L' => 4,
            'T' => 5,
            'I' => 6,
            'S' => 7,
            'z' => 8,
            'j' => 9,
            'v' => 10,
            'o' => 11,
            'l' => 12,
            't' => 13,
            'i' => 14,
            's' => 15,
            _ => panic!("Invalid piece"),
        }
    }

    pub fn new() -> Zobrist {
        let mut pieces = [[[0; 16]; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..16 {
                    pieces[i][j][k] = rand::random();
                }
            }
        }
        Zobrist {
            pieces,
            white_to_move: rand::random(),
        }
    }
    pub fn hash(&self, game: &Zjvoltis) -> u64 {
        let mut hash = 0;
        for row in 0..10 {
            for col in 0..10 {
                let piece = game.board[row][col];
                let index = Zobrist::piece_index(piece);
                hash ^= self.pieces[row][col][index];
            }
        }
        if game.white_to_move {
            hash ^= self.white_to_move;
        }
        hash
    }

    pub fn update_piece_hash(&self, hash: u64, row: usize, col: usize, piece: char) -> u64 {
        let index = Zobrist::piece_index(piece);

        hash ^ self.pieces[row][col][index]
    }

    pub fn update_white_to_move_hash(&self, hash: u64) -> u64 {
        hash ^ self.white_to_move
    }
}
