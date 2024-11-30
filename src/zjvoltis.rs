// Square values
// first bit for color
const WHITE: u8 = 0;
const BLACK: u8 = 1;
// rest of the bits for piece
const ZEBRA: u8 = 2;
const JAGUAR: u8 = 4;
const VAMPIRE: u8 = 6;
const ORANGUTAN: u8 = 8;
const LEOPARD: u8 = 10;
const TIGER: u8 = 12;
const INSECT: u8 = 14;
const SEAHORSE: u8 = 16;

// Is piece white?
fn is_white(piece: u8) -> bool {
    piece % 2 == WHITE
}

// Convert piece character to piece value
fn piece_value(piece: char) -> u8 {
    match piece {
        'Z' => WHITE | ZEBRA,
        'J' => WHITE | JAGUAR,
        'V' => WHITE | VAMPIRE,
        'O' => WHITE | ORANGUTAN,
        'L' => WHITE | LEOPARD,
        'T' => WHITE | TIGER,
        'I' => WHITE | INSECT,
        'S' => WHITE | SEAHORSE,
        'z' => BLACK | ZEBRA,
        'j' => BLACK | JAGUAR,
        'v' => BLACK | VAMPIRE,
        'o' => BLACK | ORANGUTAN,
        'l' => BLACK | LEOPARD,
        't' => BLACK | TIGER,
        'i' => BLACK | INSECT,
        's' => BLACK | SEAHORSE,
        _ => 0,
    }
}
// Convert piece value to piece character
fn piece_char(piece: u8) -> char {
    if piece % 2 == BLACK {
        return match piece & 0b11111110 {
            ZEBRA => 'z',
            JAGUAR => 'j',
            VAMPIRE => 'v',
            ORANGUTAN => 'o',
            LEOPARD => 'l',
            TIGER => 't',
            INSECT => 'i',
            SEAHORSE => 's',
            _ => '.',
        };
    } else {
        return match piece {
            ZEBRA => 'Z',
            JAGUAR => 'J',
            VAMPIRE => 'V',
            ORANGUTAN => 'O',
            LEOPARD => 'L',
            TIGER => 'T',
            INSECT => 'I',
            SEAHORSE => 'S',
            _ => '.',
        };
    }
}

#[derive(Clone, Copy)]
pub struct Zjvoltis {
    // The board is represented as a 10x10 array of characters.
    pub board: [[u8; 10]; 10],
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
        let mut board = [[0; 10]; 10];
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
                board[row][col] = piece_value(c);
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
                if self.board[row][col] == 0 {
                    empty += 1;
                } else {
                    if empty > 0 {
                        board_str.push_str(&empty.to_string());
                        empty = 0;
                    }
                    board_str.push(piece_char(self.board[row][col]));
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
                s.push(piece_char(self.board[row][col]));
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
        if piece == 0 {
            return None;
        }

        // Check that the piece belongs to the current player.
        if self.white_to_move != is_white(piece) {
            return None;
        }

        let mut squares = Vec::new();
        let mut game_over = None;
        let mut new_material = self.material;

        // Find the rest of the squares the piece occupies
        for row in 0..10 {
            for col in 0..10 {
                if !(row == m.row && col == m.col) && self.board[row][col] == piece {
                    squares.push((row, col));
                }
            }
        }

        let mut new_squares = Vec::new();

        // Rotate the piece using the squares found
        for (row, col) in &squares {
            let (new_row, new_col) = match m.hgrad {
                1 => rotate_point((*row as isize, *col as isize), (m.row, m.col)),
                2 => rotate_point(
                    rotate_point((*row as isize, *col as isize), (m.row, m.col)),
                    (m.row, m.col),
                ),
                3 => rotate_point(
                    rotate_point(
                        rotate_point((*row as isize, *col as isize), (m.row, m.col)),
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
                && piece_in_square != 0
                && self.white_to_move == is_white(piece_in_square)
            {
                return None;
            }
            new_squares.push((new_row as usize, new_col as usize));
        }

        // Perform the move.
        let mut new_board = self.board.clone();

        // Clear the old squares
        for (row, col) in &squares {
            new_board[*row][*col] = 0;
        }

        for (row, col) in new_squares {
            let piece_in_square = self.board[row][col];
            // If it's an enemy piece, remove all of it.
            if piece_in_square != 0 && self.white_to_move != is_white(piece_in_square) {
                for row in 0..10 {
                    for col in 0..10 {
                        if new_board[row][col] == piece_in_square {
                            new_board[row][col] = 0;
                            if self.white_to_move {
                                new_material += 1;
                            } else {
                                new_material -= 1;
                            }
                        }
                    }
                }
                // Game over if it was the orangutan
                if piece_in_square == WHITE | ORANGUTAN {
                    game_over = Some(-1);
                }
                if piece_in_square == BLACK | ORANGUTAN {
                    game_over = Some(1);
                }
            }

            new_board[row][col] = piece;
        }

        // check if the four center squares of the new board has the orangutan
        if new_board[4][4] == WHITE | ORANGUTAN
            && new_board[4][5] == WHITE | ORANGUTAN
            && new_board[5][4] == WHITE | ORANGUTAN
            && new_board[5][5] == WHITE | ORANGUTAN
        {
            game_over = Some(1);
        }
        if new_board[4][4] == BLACK | ORANGUTAN
            && new_board[4][5] == BLACK | ORANGUTAN
            && new_board[5][4] == BLACK | ORANGUTAN
            && new_board[5][5] == BLACK | ORANGUTAN
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
fn calculate_material(board: &[[u8; 10]; 10]) -> i32 {
    let mut score = 0;
    for row in 0..10 {
        for col in 0..10 {
            if board[row][col] != 0 {
                if board[row][col] % 2 == 1 {
                    score -= 1;
                } else {
                    score += 1;
                }
            }
        }
    }
    score
}

#[inline]
// Rotate a point around another point by 90 degrees counterclockwise
fn rotate_point((row, col): (isize, isize), (crow, ccol): (usize, usize)) -> (isize, isize) {
    (
        (col - ccol as isize + crow as isize),
        (crow as isize - row + ccol as isize),
    )
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
        assert_eq!(game.board[0][0], WHITE | ORANGUTAN);
        assert_eq!(game.board[9][9], BLACK | ORANGUTAN);
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
        assert_eq!(game2.board[4][9], WHITE | ZEBRA);
        assert_eq!(game2.board[5][9], WHITE | ZEBRA);
        assert_eq!(game2.board[4][8], WHITE | ZEBRA);
        assert_eq!(game2.board[3][8], WHITE | ZEBRA);
        assert_eq!(game2.board[2][7], 0);
        assert_eq!(game2.board[2][8], 0);
        assert_eq!(game2.board[1][7], 0);
        assert!(!game2.white_to_move);
        println!("{}", game2.to_string());
        // capture the Zebra
        let m2 = ZjvoltisMove::from_string("i63");
        let game3: Zjvoltis = game2.make_move(m2).unwrap();
        println!("{}", game3.to_string());
        assert_eq!(game3.board[4][9], 0);
        assert_eq!(game3.board[5][9], BLACK | VAMPIRE);
        assert_eq!(game3.board[6][8], BLACK | VAMPIRE);
        assert_eq!(game3.board[4][8], 0);
        assert_eq!(game3.board[3][8], 0);
        // The Zebra has been captured, so the evaluation should be -4
        assert_eq!(game3.evaluate(), -4);
        // capture the Vampire bat
        let m3 = ZjvoltisMove::from_string("j32");
        let game4: Zjvoltis = game3.make_move(m3).unwrap();
        assert_eq!(game4.board[4][9], WHITE | INSECT);
        assert_eq!(game4.board[5][9], WHITE | INSECT);
        assert_eq!(game4.board[6][9], WHITE | INSECT);
        assert_eq!(game4.board[6][8], 0);
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
