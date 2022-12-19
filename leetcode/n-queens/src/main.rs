struct Solution;

struct BoardState {
    cols: u32,
    left_dia: u32,
    right_dia: u32,
    order: Vec<usize>,
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            cols: 0,
            left_dia: 0,
            right_dia: 0,
            order: Vec::new(),
        }
    }

    pub fn possible_columns(&self, n: usize) -> Vec<usize> {
        let mut possible_indexes = Vec::new();
        let ored = self.cols | self.left_dia | self.right_dia;
        let mut mask = 1;

        for i in 0..n {
            if ored & mask == 0 {
                possible_indexes.push(i);
            }
            mask <<= 1;
        }

        possible_indexes
    }

    pub fn add_column(&mut self, col: u8) {
        // Add queen to column
        self.cols |= 1 << col;

        // Add queen to left diagonal
        self.left_dia |= 1 << col;
        self.left_dia = self.left_dia.rotate_left(1);

        // Add queen to right diagonal
        self.right_dia |= 1 << col;
        self.right_dia = self.right_dia.rotate_right(1);

        self.order.push(col as usize);
    }

    pub fn backtrace(&mut self, col: u8) {
        // Remove queen from column
        self.cols &= !(1 << col);

        // Remove queen from left diagonal
        self.left_dia = self.left_dia.rotate_right(1);
        self.left_dia &= !(1 << col);

        // Remove queen from right diagonal
        self.right_dia = self.right_dia.rotate_left(1);
        self.right_dia &= !(1 << col);

        self.order.pop();
    }

    pub fn get_solution(&self, n: u8) -> Vec<String> {
        let mut solution = Vec::new();
        let mut queen = String::new();

        for _ in 0..n {
            queen.push('.');
        }

        for col in &self.order {
            let mut this_queen = queen.clone();
            this_queen.replace_range(col..&(col + 1), "Q");
            solution.push(this_queen);
        }

        solution
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solutions = Vec::new();
        let mut state = BoardState::new();

        Self::recursion(&mut state, 0, n as usize, &mut solutions);

        solutions
    }

    fn recursion(state: &mut BoardState, row: usize, n: usize, solutions: &mut Vec<Vec<String>>) {
        // Found solution
        if row == n {
            solutions.push(state.get_solution(n as u8));
            return;
        }
        // Try each column
        for &col in &state.possible_columns(n) {
            state.add_column(col as u8);
            Self::recursion(state, row + 1, n, solutions);
            state.backtrace(col as u8);
        }
    }
}

#[cfg(test)]
mod state_tests {
    use super::BoardState;

    #[test]
    fn add_column_test0() {
        let mut board = BoardState::new();
        board.add_column(0);
        assert_eq!(board.cols, 1);
        assert_eq!(board.left_dia, 2);
        assert_eq!(board.right_dia, 1 << 31);
    }

    #[test]
    fn add_column_test1() {
        let mut board = BoardState::new();
        board.add_column(1);
        assert_eq!(board.cols, 2);
        assert_eq!(board.left_dia, 4);
        assert_eq!(board.right_dia, 1);
    }

    #[test]
    fn add_column_test2() {
        let mut board = BoardState::new();
        board.add_column(2);
        assert_eq!(board.cols, 4);
        assert_eq!(board.left_dia, 8);
        assert_eq!(board.right_dia, 2);
    }

    #[test]
    fn add_column_test3() {
        let mut board = BoardState::new();
        board.add_column(3);
        assert_eq!(board.cols, 8);
        assert_eq!(board.left_dia, 16);
        assert_eq!(board.right_dia, 4);
    }

    #[test]
    fn possible_columns_test1() {
        let mut board = BoardState::new();
        board.add_column(0);
        assert_eq!(board.possible_columns(4), vec![2, 3])
    }

    #[test]
    fn possible_columns_test2() {
        let mut board = BoardState::new();
        board.add_column(0);
        board.add_column(2);
        assert_eq!(board.possible_columns(4), vec![])
    }

    #[test]
    fn backtrace_test1() {
        let mut board = BoardState::new();
        board.add_column(0);
        board.add_column(2);
        board.backtrace(2);
        assert_eq!(board.cols, 1);
        assert_eq!(board.left_dia, 2);
        assert_eq!(board.right_dia, 1 << 31);
    }

    #[test]
    fn backtrace_test2() {
        let mut board = BoardState::new();
        // .Q..
        // ....
        // ....
        // ....
        board.add_column(1);
        assert_eq!(board.cols, 0b0010);
        assert_eq!(board.left_dia, 0b0100);
        assert_eq!(board.right_dia, 0b0001);
        // .Q..
        // ...Q
        // ....
        // ....
        board.add_column(3);
        assert_eq!(board.cols, 0b1010);
        assert_eq!(board.left_dia, 0b11000);
        assert_eq!(board.right_dia, 0b10000000000000000000000000000100);
        // .Q..
        // ...Q
        // Q...
        // ....
        board.add_column(0);
        assert_eq!(board.cols, 0b1011);
        assert_eq!(board.left_dia, 0b110010);
        assert_eq!(board.right_dia, 0b11000000000000000000000000000010);
        // .Q..
        // ...Q
        // Q...
        // ..Q.
        board.add_column(2);
        assert_eq!(board.cols, 0b1111);
        assert_eq!(board.left_dia, 0b1101100);
        assert_eq!(board.right_dia, 0b01100000000000000000000000000011);

        let string_board_state = vec![".Q..", "...Q", "Q...", "..Q."];
        assert_eq!(board.get_solution(4), string_board_state);
        // .Q..
        // ...Q
        // Q...
        // ....
        board.backtrace(2);
        assert_eq!(board.cols, 0b1011);
        assert_eq!(board.left_dia, 0b110010);
        assert_eq!(board.right_dia, 0b11000000000000000000000000000010);
        // .Q..
        // ...Q
        // ....
        // ....
        board.backtrace(0);
        assert_eq!(board.cols, 0b1010);
        assert_eq!(board.left_dia, 0b11000);
        assert_eq!(board.right_dia, 0b10000000000000000000000000000100);
        // .Q..
        // ....
        // ....
        // ....
        board.backtrace(3);
        assert_eq!(board.cols, 0b0010);
        assert_eq!(board.left_dia, 0b0100);
        assert_eq!(board.right_dia, 0b0001);
        // ....
        // ....
        // ....
        // ....
        board.backtrace(1);
        assert_eq!(board.cols, 0);
        assert_eq!(board.left_dia, 0);
        assert_eq!(board.right_dia, 0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"],]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::solve_n_queens(2), Vec::<Vec<String>>::new());
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::solve_n_queens(3), Vec::<Vec<String>>::new());
    }

    #[test]
    fn test4() {
        let mut solution = vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."],
        ];
        let mut calc = Solution::solve_n_queens(4);
        solution.sort();
        calc.sort();
        assert_eq!(solution, calc);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::solve_n_queens(5).len(), 10);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::solve_n_queens(6).len(), 4);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::solve_n_queens(7).len(), 40);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::solve_n_queens(9).len(), 352);
    }

    #[test]
    fn test10() {
        assert_eq!(Solution::solve_n_queens(10).len(), 724);
    }

    #[test]
    fn test11() {
        assert_eq!(Solution::solve_n_queens(11).len(), 2680);
    }
    #[test]
    fn test12() {
        assert_eq!(Solution::solve_n_queens(12).len(), 14200);
    }
    #[test]
    fn test13() {
        assert_eq!(Solution::solve_n_queens(13).len(), 73712);
    }
    #[test]
    fn test14() {
        assert_eq!(Solution::solve_n_queens(14).len(), 365596);
    }
    #[test]
    fn test15() {
        assert_eq!(Solution::solve_n_queens(15).len(), 2279184);
    }
}
