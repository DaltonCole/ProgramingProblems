struct Solution;

struct BoardState {
    rows: [u16; 9],
    cols: [u16; 9],
    square: [u16; 9],
    board: [[u8; 9]; 9],
    already_tried: [[u16; 9]; 9],
}

impl BoardState {
    pub fn new(char_board: &Vec<Vec<char>>) -> BoardState {
        let mut rows = [0; 9];
        let mut cols = [0; 9];
        let mut square = [0; 9];
        let mut board = [[0; 9]; 9];
        let already_tried = [[0; 9]; 9];

        for (r_index, row) in char_board.iter().enumerate() {
            for (c_index, &ele) in row.iter().enumerate() {
                if let Some(num) = ele.to_digit(10) {
                    let bit_num = 1 << (num - 1);
                    rows[r_index] |= bit_num;
                    cols[c_index] |= bit_num;

                    let square_index = (3 * (r_index / 3)) + (c_index / 3);
                    square[square_index] |= bit_num;

                    board[r_index][c_index] = num as u8;
                }
            }
        }

        BoardState {
            rows,
            cols,
            square,
            board,
            already_tried,
        }
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

    fn count_solved_boxes(&self) -> usize {
        self.board.iter().flatten().filter(|&n| *n != 0).count()
    }

    /// Get the three numbers that represent a element in the sodoku puzzle
    fn get_row_col_square(&self, row_num: usize, col_num: usize) -> (u16, u16, u16) {
        let row = self.rows[row_num];
        let col = self.cols[col_num];
        let square_num = (3 * (row_num / 3)) + (col_num / 3);
        let square = self.square[square_num];

        (row, col, square)
    }

    /// Get the internal representation of the possible number for a given row/col
    pub fn possible_numbers(&self, row_num: usize, col_num: usize) -> u16 {
        let (row, col, square) = self.get_row_col_square(row_num, col_num);
        let already_tried = self.already_tried[row_num][col_num];

        row | col | square | already_tried
    }

    /// Get number of possible elements that could be in this box
    pub fn num_possible_numbers(&self, row_num: usize, col_num: usize) -> usize {
        9 - u16::count_ones(self.possible_numbers(row_num, col_num)) as usize
    }

    pub fn get_first_possible_num(&self, row_num: usize, col_num: usize) -> u8 {
        self.possible_numbers(row_num, col_num).trailing_ones() as u8 + 1
    }

    /// Get a vector of possible results in integer format
    pub fn possible_numbers_vec(&self, row_num: usize, col_num: usize) -> Vec<u8> {
        let mut possible_numbers = self.possible_numbers(row_num, col_num);
        let mut nums = Vec::new();

        for shift in 1..=9 {
            //if possible_numbers & 1 == 1 {
            if possible_numbers.trailing_zeros() >= 1 {
                nums.push(shift);
            }
            possible_numbers >>= 1;
        }

        nums
    }

    /// Returns true if the puzzle has been solved
    pub fn solved(&self) -> bool {
        let goal_num = ((1u32 << 9) - 1) as u16;
        for square in self.square {
            if square != goal_num {
                return false;
            }
        }
        true
    }

    /// Finds the best box and value given a board state
    pub fn get_best_box_and_value(&self) -> ((usize, usize), u8) {
        let mut best_row = 0;
        let mut best_col = 0;
        let mut best_num = 0;
        let mut best_possibilities = 10;

        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    let num_possible = self.num_possible_numbers(row, col);
                    if num_possible > 0 && num_possible < best_possibilities {
                        best_row = row;
                        best_col = col;
                        best_num = self.get_first_possible_num(row, col);
                        best_possibilities = num_possible;

                        if best_possibilities == 1 {
                            return ((best_row, best_col), best_num);
                        }
                    }
                }
            }
        }
        return ((best_row, best_col), best_num);
    }

    pub fn make_move(&mut self, row: usize, col: usize, num: u8) {
        self.board[row][col] = num;

        let bit_num = 1 << (num - 1);
        self.rows[row] |= bit_num;
        self.cols[col] |= bit_num;
        self.square[(3 * (row / 3)) + (col / 3)] |= bit_num;
        self.already_tried[row][col] |= bit_num;
    }

    pub fn undo_move(&mut self, row: usize, col: usize, previous: Option<((usize, usize), u8)>) {
        let num = self.board[row][col];

        self.board[row][col] = 0;
        let bit_num = !(1 << (num - 1));
        self.rows[row] &= bit_num;
        self.cols[col] &= bit_num;
        self.square[(3 * (row / 3)) + (col / 3)] &= bit_num;

        if let Some(((prev_row, prev_col), prev_num)) = previous {
            self.already_tried[prev_row][prev_col] &= !(1 << (prev_num - 1));
        }
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_state = BoardState::new(&board);
        let mut state_stack = Vec::new();
        let mut prev_state = None;
        let mut level = 0;
        let mut last_level = 0;
        let mut last_row = 0;
        let mut last_col = 0;
        let mut last_num = 0;
        let mut a = 0;

        while !board_state.solved() {
            a += 1;
            if a > 10000 {
                break;
            }
            let ((mut best_row, mut best_col), mut num) = board_state.get_best_box_and_value();

            dbg!(state_stack.len());
            dbg!(board_state.get_best_box_and_value());
            // TODO: I made a mess, fix it. Something about undoing prevous moves
            // If best move has a num of 0, there are no possible moves, so undo last move
            while num == 0 {
                ((last_row, last_col), last_num, last_level) = state_stack.pop().unwrap();
                board_state.undo_move(last_row, last_col, prev_state);
                if last_level > board_state.count_solved_boxes() {
                    prev_state = Some(((last_row, last_col), last_num));
                } else {
                    prev_state = None;
                }

                ((best_row, best_col), num) = board_state.get_best_box_and_value();
                dbg!(board_state.get_best_box_and_value());
            }

            board_state.make_move(best_row, best_col, num);
            state_stack.push(((best_row, best_col), num, board_state.count_solved_boxes()));
        }

        board_state.convert(board);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_sate_test() {
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

        // Check possible states
        assert_eq!(vec![1, 2, 4, 5], board_state.possible_numbers_vec(8, 1));
        assert_eq!(vec![1, 2], board_state.possible_numbers_vec(0, 0));

        // Check number of possible states
        assert_eq!(4, board_state.num_possible_numbers(8, 1));
        assert_eq!(2, board_state.num_possible_numbers(0, 0));

        // See if convert works
        let mut mut_board = board.clone();
        board_state.convert(&mut mut_board);
        assert_eq!(board, mut_board);

        // Test get first possible num
        assert_eq!(5, board_state.get_first_possible_num(4, 4));

        // Best box and num
        assert_eq!(((4, 4), 5), board_state.get_best_box_and_value());

        // Test undo
        board_state.undo_move(0, 0, None);
        assert_eq!(0, board_state.board[0][0]);

        // Test solved
        let solved_board = BoardState::new(&goal);
        assert!(!board_state.solved());
        assert!(solved_board.solved());
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
