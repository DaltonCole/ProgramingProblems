struct Solution;

use std::collections::HashSet;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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

    fn count_solved_boxes(&self) -> usize {
        self.board.iter().flatten().filter(|&n| *n != 0).count()
    }

    pub fn is_valid(&self) -> bool {
        let mut row;
        let mut col;
        let mut square;

        for this_row in self.board.iter() {
            row = 0;
            for &ele in this_row.iter() {
                if ele == 0 {
                    continue;
                }
                let num = 1 << ele;
                if (row & num) != 0 {
                    return false;
                }
                row |= num;
            }
        }

        for this_col in 0..9 {
            col = 0;
            for this_row in 0..9 {
                if self.board[this_row][this_col] == 0 {
                    continue;
                }
                let num = 1 << self.board[this_row][this_col];
                if (col & num) != 0 {
                    return false;
                }
                col |= num;
            }
        }

        for square_row in 0..3 {
            for square_col in 0..3 {
                square = 0;
                for row_i in 0..3 {
                    for col_j in 0..3 {
                        if self.board[(3 * square_row) + row_i][(3 * square_col) + col_j] == 0 {
                            continue;
                        }
                        let num =
                            1 << self.board[(3 * square_row) + row_i][(3 * square_col) + col_j];
                        if (square & num) != 0 {
                            return false;
                        }
                        square |= num;
                    }
                }
            }
        }

        true
    }

    /// Returns true if the puzzle has been solved
    pub fn solved(&self) -> bool {
        self.count_solved_boxes() == 9 * 9 && self.is_valid()
    }

    fn get_row_col_square_state(&self, row_i: usize, col_j: usize) -> u16 {
        let mut row: u16 = 0;
        let mut col: u16 = 0;
        let mut square: u16 = 0;

        for i in 0..9 {
            row |= 1 << self.board[row_i][i];
            col |= 1 << self.board[i][col_j];
        }

        for i in 0..3 {
            for j in 0..3 {
                square |= 1 << self.board[(3 * (row_i / 3)) + i][(3 * (col_j / 3)) + j];
            }
        }

        //dbg!(format!("{:b}", (row | col | square) >> 1));

        (row | col | square) >> 1
    }

    fn get_num_possibilities(&self, internal_rep: u16) -> u8 {
        9 - internal_rep.count_ones() as u8
    }

    fn get_possibilities(&self, mut internal_rep: u16) -> Vec<u8> {
        let mut poss = Vec::new();
        for i in 1..=9 {
            if (internal_rep & 1) == 0 {
                poss.push(i);
            }
            internal_rep >>= 1;
        }
        poss
    }

    pub fn make_move(&mut self, row: usize, col: usize, num: u8) {
        self.board[row][col] = num;
    }

    pub fn undo_move(&mut self, row: usize, col: usize) {
        self.board[row][col] = 0;
    }

    fn next_move(&mut self, explored: &HashSet<BoardState>) -> Option<((usize, usize), u8)> {
        let mut best_row = 0;
        let mut best_col = 0;
        let mut best_num = 0;
        let mut num_possible = 10;

        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    let internal_rep = self.get_row_col_square_state(row, col);
                    let box_num_possible = self.get_num_possibilities(internal_rep);
                    if box_num_possible < num_possible {
                        let possible_nums = self.get_possibilities(internal_rep);
                        for num in possible_nums {
                            self.board[row][col] = num;
                            if !explored.contains(&self) {
                                best_row = row;
                                best_col = col;
                                best_num = num;
                                num_possible = box_num_possible;
                                self.board[row][col] = 0;
                                break;
                            }
                            self.board[row][col] = 0;
                        }
                    }
                }
            }
        }

        if best_num == 0 {
            None
        } else {
            Some(((best_row, best_col), best_num))
        }
    }

    pub fn solve(char_board: &Vec<Vec<char>>) -> BoardState {
        let mut board = BoardState::new(&char_board);
        board.print();
        println!("");
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let mut explored = HashSet::new();
        let mut last_row;
        let mut last_col;

        while !board.solved() {
            // Get next move
            match board.next_move(&explored) {
                Some(((row, col), num)) => {
                    // Make the next move
                    board.make_move(row, col, num);
                    // Add new board to the explored state
                    explored.insert(board.clone());
                    // Add move to the stack of moves to get to this state
                    moves.push((row, col));
                    //dbg!(format!("{} {} {}", row, col, num));
                }
                None => {
                    // Undo last move
                    (last_row, last_col) = moves.pop().unwrap();
                    board.undo_move(last_row, last_col);
                }
            }

            //dbg!(moves.len());
        }

        board.print();

        board
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let board_state = BoardState::solve(&board);

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
        // Get possible_nums
        let internal_rep = board_state.get_row_col_square_state(8, 1);
        assert_eq!(4, board_state.get_num_possibilities(internal_rep));
        assert_eq!("111100100", format!("{:b}", internal_rep));
        assert_eq!(
            vec![1, 2, 4, 5],
            board_state.get_possibilities(internal_rep)
        );

        let internal_rep = board_state.get_row_col_square_state(0, 0);
        assert_eq!(2, board_state.get_num_possibilities(internal_rep));
        assert_eq!(vec![1, 2], board_state.get_possibilities(internal_rep));

        // See if convert works
        let mut mut_board = board.clone();
        board_state.convert(&mut mut_board);
        assert_eq!(board, mut_board);

        // Test undo
        board_state.undo_move(0, 0);
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
