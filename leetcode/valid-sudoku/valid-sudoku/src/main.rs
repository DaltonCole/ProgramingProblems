struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Row
        for row in 0..9 {
            let mut occurred = [false; 9];
            for col in 0..9 {
                let ele = board[row][col];
                if ele != '.' {
                    let num = (ele.to_digit(10).unwrap() - 1) as usize;
                    if occurred[num] == true {
                        return false;
                    }
                    occurred[num] = true;
                }
            }
        }
        // Col
        for col in 0..9 {
            let mut occurred = [false; 9];
            for row in 0..9 {
                let ele = board[row][col];
                if ele != '.' {
                    let num = (ele.to_digit(10).unwrap() - 1) as usize;
                    if occurred[num] == true {
                        return false;
                    }
                    occurred[num] = true;
                }
            }
        }
        // 3x3
        for row_grid in 0..3 {
            for col_grid in 0..3 {
                let mut occurred = [false; 9];
                for row in 0..3 {
                    for col in 0..3 {
                        let row_offset = (3 * row_grid) + row;
                        let col_offset = (3 * col_grid) + col;
                        let ele = board[row_offset][col_offset];
                        if ele != '.' {
                            let num = (ele.to_digit(10).unwrap() - 1) as usize;
                            if occurred[num] == true {
                                return false;
                            }
                            occurred[num] = true;
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            true,
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            false,
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            false,
            Solution::is_valid_sudoku(vec![
                vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
                vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
                vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
                vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
                vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
                vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
                vec!['.', '.', '4', '.', '.', '.', '.', '.', '.']
            ])
        );
    }
}
