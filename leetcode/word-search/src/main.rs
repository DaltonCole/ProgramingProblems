struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        //let mut used_tiles = Vec::new();

        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == word.chars().next().unwrap() {
                    if Self::exist_helper(&board, &word[1..], &mut vec![(i, j)]) == true {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn exist_helper(
        board: &Vec<Vec<char>>,
        remaining_word: &str,
        used_tiles: &mut Vec<(usize, usize)>,
    ) -> bool {
        if remaining_word.len() == 0 {
            return true;
        }
        let next_char = remaining_word.chars().next().unwrap();
        for (next_x, next_y) in Self::legal_tiles(&board, &used_tiles, next_char) {
            used_tiles.push((next_x, next_y));
            if Self::exist_helper(&board, &remaining_word[1..], used_tiles) == true {
                return true;
            }
            used_tiles.pop();
        }
        false
    }

    fn legal_tiles(
        board: &Vec<Vec<char>>,
        used_tiles: &Vec<(usize, usize)>,
        next_char: char,
    ) -> Vec<(usize, usize)> {
        if used_tiles.len() == 0 {
            panic!("Must have at least 1 used tile");
        }
        let mut new_tiles = Vec::new();
        let &(x, y) = used_tiles.last().unwrap();

        // Up
        if (x > 0)
            && used_tiles
                .iter()
                .find(|&&(a, b)| a == x - 1 && b == y)
                .is_none()
            && board[x - 1][y] == next_char
        {
            new_tiles.push((x - 1, y));
        }
        // Down
        if (x < board.len() - 1)
            && used_tiles
                .iter()
                .find(|&&(a, b)| a == x + 1 && b == y)
                .is_none()
            && board[x + 1][y] == next_char
        {
            new_tiles.push((x + 1, y));
        }
        // Left
        if (y > 0)
            && used_tiles
                .iter()
                .find(|&&(a, b)| a == x && b == y - 1)
                .is_none()
            && board[x][y - 1] == next_char
        {
            new_tiles.push((x, y - 1));
        }
        // Right
        if (y < board[0].len() - 1)
            && used_tiles
                .iter()
                .find(|&&(a, b)| a == x && b == y + 1)
                .is_none()
            && board[x][y + 1] == next_char
        {
            new_tiles.push((x, y + 1));
        }

        new_tiles
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ));
    }
    #[test]
    fn test3() {
        assert!(!Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ));
    }
}
