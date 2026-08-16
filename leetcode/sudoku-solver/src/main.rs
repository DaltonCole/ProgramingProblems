struct Solution;

struct BoardState {
    board: [[u8; 9]; 9],
}

impl BoardState {
    pub fn new(char_board: &Vec<Vec<char>>) -> BoardState {
        let mut board = [[0; 9]; 9];

        for (r_index, row) in char_board.iter().enumerate() {
            for (c_index, &ele) in row.iter().enumerate() {
                if let Some(num) = ele.to_digit(10) {
                    board[r_index][c_index] = num as u8;
                }
            }
        }

        BoardState { board }
    }

    /// Print the board to stdout. Board looks like a sodoku board.
    pub fn print(&self) {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] != 0 {
                    print!("{}", self.board[row][col]);
                } else {
                    print!("*");
                }

                if col % 3 == 2 && col < 8 {
                    print!("|");
                }
            }
            println!("");
            if row % 3 == 2 && row < 8 {
                println!("{}", "-".repeat(11));
            }
        }
    }

    /// Convert the sodoku board from int to char format.
    pub fn convert(&self, other_board: &mut Vec<Vec<char>>) {
        for row in 0..9 {
            for col in 0..9 {
                other_board[row][col] =
                    std::char::from_digit(self.board[row][col] as u32, 10).unwrap();

                if other_board[row][col] == '0' {
                    other_board[row][col] = '.';
                }
            }
        }
    }

    fn is_valid_move(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..9 {
            if self.board[row][i] != 0 && self.board[row][i] == num {
                return false;
            }
            if self.board[i][col] != 0 && self.board[i][col] == num {
                return false;
            }
            let s_row = (3 * (row / 3)) + (i / 3);
            let s_col = (3 * (col / 3)) + (i % 3);
            if self.board[s_row][s_col] != 0 && self.board[s_row][s_col] == num {
                return false;
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    for num in 1..=9 {
                        if self.is_valid_move(row, col, num) {
                            self.board[row][col] = num;

                            if self.solve() {
                                return true;
                            } else {
                                self.board[row][col] = 0;
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_state = BoardState::new(&board);
        board_state.solve();

        board_state.convert(board);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_state_test() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let goal = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        // Convert board to state struct
        let mut board_state = BoardState::new(&board);

        board_state.print();

        // See if convert works
        let mut mut_board = board.clone();
        board_state.convert(&mut mut_board);
        assert_eq!(board, mut_board);
    }

    #[test]
    fn test1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let goal = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        Solution::solve_sudoku(&mut board);

        assert_eq!(goal, board);
    }
    #[test]
    fn test2() {
        let mut board = vec![
            vec!['.', '.', '9', '7', '4', '8', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '2', '.', '1', '.', '9', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '.', '2', '4', '.'],
            vec!['.', '6', '4', '.', '1', '.', '5', '9', '.'],
            vec!['.', '9', '8', '.', '.', '.', '3', '.', '.'],
            vec!['.', '.', '.', '8', '.', '3', '.', '2', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '2', '7', '5', '9', '.', '.'],
        ];

        let goal = vec![
            vec!['5', '1', '9', '7', '4', '8', '6', '3', '2'],
            vec!['7', '8', '3', '6', '5', '2', '4', '1', '9'],
            vec!['4', '2', '6', '1', '3', '9', '8', '7', '5'],
            vec!['3', '5', '7', '9', '8', '6', '2', '4', '1'],
            vec!['2', '6', '4', '3', '1', '7', '5', '9', '8'],
            vec!['1', '9', '8', '5', '2', '4', '3', '6', '7'],
            vec!['9', '7', '5', '8', '6', '3', '1', '2', '4'],
            vec!['8', '3', '2', '4', '9', '1', '7', '5', '6'],
            vec!['6', '4', '1', '2', '7', '5', '9', '8', '3'],
        ];
        Solution::solve_sudoku(&mut board);

        assert_eq!(goal, board);
    }
}
