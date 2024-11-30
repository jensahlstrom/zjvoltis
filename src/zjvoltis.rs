#[derive(Clone, Copy)]
pub struct Zjvoltis {
    // The board is represented as a 10x10 array of characters.
    pub board: [[char; 10]; 10],
    pub white_to_move: bool,
    pub game_over: Option<i32>,
    // difference in material (positive for white, negative for black)
    pub material: i32,
}

impl Zjvoltis {
    // Parse a FEN representation of the board
    pub fn from_fen(s: &str) -> Zjvoltis {
        // Split the FEN string into the board and the player to move
        let mut parts = s.split(' ');
        let board_str = parts.next().unwrap();
        let player_to_move = parts.next().unwrap();

        let mut row = 9;
        let mut col = 0;
        let mut board = [['.'; 10]; 10];
        for c in board_str.chars() {
            if c.is_numeric() {
                col += c.to_digit(10).unwrap() as usize;
            } else if c == 'A' {
                col += 10;
            } else if c == '.' {
                col += 1;
            } else if c == '/' {
                row -= 1;
                col = 0;
            } else {
                board[row][col] = c;
                col += 1;
            }
        }

        Zjvoltis {
            board,
            white_to_move: player_to_move == "w",
            game_over: None,
            material: calculate_material(&board),
        }
    }

    // Output a FEN representation of the board
    pub fn to_fen(&self) -> String {
        let mut board_str = String::new();
        for row in (0..10).rev() {
            let mut empty = 0;
            for col in 0..10 {
                if self.board[row][col] == '.' {
                    empty += 1;
                } else {
                    if empty > 0 {
                        board_str.push_str(&empty.to_string());
                        empty = 0;
                    }
                    board_str.push(self.board[row][col]);
                }
            }
            if empty == 10 {
                board_str.push('A');
            } else if empty > 0 {
                board_str.push_str(&empty.to_string());
            }
            if row > 0 {
                board_str.push('/');
            }
        }

        format!(
            "{} {}",
            board_str,
            if self.white_to_move { "w" } else { "b" }
        )
    }

    // Output as a printed board
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for row in (0..10).rev() {
            for col in 0..10 {
                s.push(self.board[row][col]);
            }
            s.push('\n');
        }
        s
    }

    // The initial board
    pub fn new() -> Zjvoltis {
        Zjvoltis::from_fen("i.ll.jj.oo/i.zl.js.oo/izzltjssvv/iz1ttt1sv///.VS.TTT1ZI/VVSSJTLZZI/OO.SJ.LZ.I/OO.JJ.LL.I w")
    }

    // Make a move on the board
    pub fn make_move(&self, m: ZjvoltisMove) -> Option<Zjvoltis> {
        // Check if the move is within the bounds of the board.
        if m.row > 9 || m.col > 9 {
            return None;
        }
        // Check if the rotation is an allowed value.
        if m.hgrad < 1 || m.hgrad > 3 {
            return None;
        }

        // Check that there is a piece on the board at the given square.
        let piece = self.board[m.row][m.col];
        if piece == '.' {
            return None;
        }

        // Check that the piece belongs to the current player.
        if self.white_to_move == piece.is_lowercase() {
            return None;
        }

        // Perform the move.
        let mut new_board = self.board.clone();
        let mut squares = Vec::new();
        let mut game_over = None;
        let mut new_material = self.material;

        // Find the rest of the squares the piece occupies
        for row in 0..10 {
            for col in 0..10 {
                if !(row == m.row && col == m.col) && self.board[row][col] == piece {
                    squares.push((row, col));
                    // Clear square from new board
                    new_board[row][col] = '.';
                }
            }
        }
        // Rotate the piece using the squares found
        for (row, col) in squares {
            let (new_row, new_col) = match m.hgrad {
                1 => rotate_point((row as isize, col as isize), (m.row, m.col)),
                2 => rotate_point(
                    rotate_point((row as isize, col as isize), (m.row, m.col)),
                    (m.row, m.col),
                ),
                3 => rotate_point(
                    rotate_point(
                        rotate_point((row as isize, col as isize), (m.row, m.col)),
                        (m.row, m.col),
                    ),
                    (m.row, m.col),
                ),
                _ => (0, 0),
            };
            // Check if the new square is within the bounds of the board.
            if new_row < 0 || new_row > 9 || new_col < 0 || new_col > 9 {
                return None;
            }
            let piece_in_square = self.board[new_row as usize][new_col as usize];
            // If it's our own piece, we can't move there.
            if piece_in_square != piece
                && piece_in_square != '.'
                && self.white_to_move == piece_in_square.is_uppercase()
            {
                return None;
            }
            // If it's an enemy piece, remove all of it.
            if piece_in_square != '.' && self.white_to_move == piece_in_square.is_lowercase() {
                for row in 0..10 {
                    for col in 0..10 {
                        if new_board[row][col] == piece_in_square {
                            new_board[row][col] = '.';
                            if self.white_to_move {
                                new_material += 1;
                            } else {
                                new_material -= 1;
                            }
                        }
                    }
                }
                // Game over if it was the orangutan
                if piece_in_square == 'O' {
                    game_over = Some(-1);
                }
                if piece_in_square == 'o' {
                    game_over = Some(1);
                }
            }

            new_board[new_row as usize][new_col as usize] = piece;
        }

        // check if the four center squares of the new board has the orangutan
        if new_board[4][4] == 'O'
            && new_board[4][5] == 'O'
            && new_board[5][4] == 'O'
            && new_board[5][5] == 'O'
        {
            game_over = Some(1);
        }
        if new_board[4][4] == 'o'
            && new_board[4][5] == 'o'
            && new_board[5][4] == 'o'
            && new_board[5][5] == 'o'
        {
            game_over = Some(-1);
        }

        Some(Zjvoltis {
            board: new_board,
            white_to_move: !self.white_to_move,
            game_over,
            material: new_material,
        })
    }

    // Generate valid moves for the current player.
    pub fn generate_moves(&self) -> Vec<(ZjvoltisMove, Zjvoltis)> {
        let mut moves = Vec::new();
        for row in 0..10 {
            for col in 0..10 {
                for hgrad in 1..3 {
                    let m = ZjvoltisMove { row, col, hgrad };
                    let board = self.make_move(m);
                    if board.is_some() {
                        moves.push((m, board.unwrap()));
                    }
                }
            }
        }
        moves
    }

    pub fn evaluate(&self) -> i32 {
        if self.game_over.is_some() {
            return self.game_over.unwrap() << 10;
        }
        self.material
    }
}

// Calculate the material, counting +1 for each square white has a piece on, and -1 for each square black has a piece on.
fn calculate_material(board: &[[char; 10]; 10]) -> i32 {
    let mut score = 0;
    for row in 0..10 {
        for col in 0..10 {
            if board[row][col].is_uppercase() {
                score += 1;
            } else if board[row][col].is_lowercase() {
                score -= 1;
            }
        }
    }
    score
}

// Rotate a point around another point by 90 degrees counterclockwise
fn rotate_point((row, col): (isize, isize), (crow, ccol): (usize, usize)) -> (isize, isize) {
    let (zrow, zcol) = (row - crow as isize, col - ccol as isize);
    let (nzrow, nzcol) = (zcol, -zrow);
    ((nzrow + crow as isize), (nzcol + ccol as isize))
}

#[derive(Clone, Copy)]
pub struct ZjvoltisMove {
    // 0-9 for the row
    pub row: usize,
    // 0-9 for the col (A-J)
    pub col: usize,
    // 1-3 for the rotation in hectogradians, counterclockwise
    pub hgrad: usize,
}

impl ZjvoltisMove {
    // Create a move from a string like "e51"
    pub fn from_string(s: &str) -> ZjvoltisMove {
        let mut chars = s.chars();
        let col = chars.next().unwrap() as usize - 'a' as usize;
        let row = chars.next().unwrap() as usize - '0' as usize;
        let hgrad = chars.next().unwrap() as usize - '0' as usize;
        ZjvoltisMove { row, col, hgrad }
    }

    // Output a move as a string like "e51"
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            (self.col as u8 + 'a' as u8) as char,
            (self.row as u8 + '0' as u8) as char,
            self.hgrad
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Zjvoltis::new();
        assert!(game.white_to_move);
        assert_eq!(game.board[0][0], 'O');
    }

    #[test]
    fn test_fen() {
        let game = Zjvoltis::new();
        let fen = game.to_fen();
        assert_eq!(fen, "i1ll1jj1oo/i1zl1js1oo/izzltjssvv/iz1ttt1sv1/A/A/1VS1TTT1ZI/VVSSJTLZZI/OO1SJ1LZ1I/OO1JJ1LL1I w");
        let game2 = Zjvoltis::from_fen(&fen);
        assert_eq!(game.board, game2.board);
        assert_eq!(game.white_to_move, game2.white_to_move);
    }

    #[test]
    fn test_make_moves() {
        let game = Zjvoltis::new();
        // Move the Zebra
        let m1 = ZjvoltisMove::from_string("i32");
        let game2 = game.make_move(m1).unwrap();
        assert_eq!(game2.board[4][9], 'Z');
        assert_eq!(game2.board[5][9], 'Z');
        assert_eq!(game2.board[4][8], 'Z');
        assert_eq!(game2.board[3][8], 'Z');
        assert_eq!(game2.board[2][7], '.');
        assert_eq!(game2.board[2][8], '.');
        assert_eq!(game2.board[1][7], '.');
        assert!(!game2.white_to_move);
        // capture the Zebra
        let m2 = ZjvoltisMove::from_string("i63");
        let game3: Zjvoltis = game2.make_move(m2).unwrap();
        assert_eq!(game3.board[4][9], '.');
        assert_eq!(game3.board[5][9], 'v');
        assert_eq!(game3.board[6][8], 'v');
        assert_eq!(game3.board[4][8], '.');
        assert_eq!(game3.board[3][8], '.');
        // The Zebra has been captured, so the evaluation should be -4
        assert_eq!(game3.evaluate(), -4);
        // capture the Vampire bat
        let m3 = ZjvoltisMove::from_string("j32");
        let game4: Zjvoltis = game3.make_move(m3).unwrap();
        assert_eq!(game4.board[4][9], 'I');
        assert_eq!(game4.board[5][9], 'I');
        assert_eq!(game4.board[6][9], 'I');
        assert_eq!(game4.board[6][8], '.');
        // The vampire bat has been captured, so the evaluation should be -1
        assert_eq!(game4.evaluate(), -1);
        // Move the tiger
        let m4 = ZjvoltisMove::from_string("e62");
        let game5: Zjvoltis = game4.make_move(m4).unwrap();
        // Move the insect, capture the orangutan, game should be over
        print!("{}", game5.to_string());
        let m5 = ZjvoltisMove::from_string("j62");
        let game6: Zjvoltis = game5.make_move(m5).unwrap();
        assert_eq!(game6.game_over, Some(1));
    }

    #[test]
    fn test_illegal_move_own_piece() {
        let game = Zjvoltis::new();
        let m1 = ZjvoltisMove::from_string("h31");
        let game2 = game.make_move(m1);
        assert!(game2.is_none());
    }

    #[test]
    fn test_illegal_move_out_of_bounds() {
        let game = Zjvoltis::new();
        let m1 = ZjvoltisMove::from_string("i41");
        let game2 = game.make_move(m1);
        assert!(game2.is_none());
    }

    #[test]
    fn test_initial_moves() {
        let game = Zjvoltis::new();
        let moves = game.generate_moves();
        // There are ten legal moves for the white player
        assert_eq!(moves.len(), 10);
    }

    #[test]
    fn test_initial_position_is_zero() {
        let game = Zjvoltis::new();
        assert_eq!(game.evaluate(), 0);
    }
}
