use std::env;
use std::fs;
use std::io;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn convert(s: &String) -> (Vec<u32>, Vec<Bingo>) {
    let mut lines = s.lines();
    let nums: Vec<u32> = lines.next()
        .ok_or("Bad").unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut bingo_boards = Vec::<Bingo>::new();
    for line in lines {
        if line == "" {
            bingo_boards.push(Bingo::new());
        } else {
            if let Some(last) = bingo_boards.last_mut() {
                (*last).board.push(line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect());
                (*last).markers.push(line.split_whitespace()
                    .map(|_| false)
                    .collect());

            }
        }
    }

    (nums, bingo_boards)
}

fn mark_tiles(nums: &Vec<u32>, boards: &mut Vec<Bingo>) -> (u32, usize) { // Winner, board index
    for &value in nums {
        for (index, board) in boards.iter_mut().enumerate() {
            if board.mark_tile(value) == true {
                if board.won() == true {
                    return (value, index);
                }
            }
        }
    }
    return (0, 0);
}

fn mark_tiles_part2(nums: &Vec<u32>, boards: &mut Vec<Bingo>) -> (u32, usize) { // Loser, board index
    let mut won_boards = vec![false; boards.len()];
    for &value in nums {
        for (index, board) in boards.iter_mut().enumerate() {
            if board.mark_tile(value) == true {
                if board.won() == true {
                    won_boards[index] = true;
                    if won_boards.iter().filter(|&&x| x == false).count() == 0 {
                        return (value, index);
                    }
                }
            }
        }
    }
    return (0, 0);
}

struct Bingo {
    pub board: Vec<Vec<u32>>,
    pub markers: Vec<Vec<bool>>,
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            board: Vec::<Vec<u32>>::new(), 
            markers: Vec::<Vec<bool>>::new(),
        } 
    }

    fn mark_tile(&mut self, num: u32) -> bool {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == num {
                    self.markers[i][j] = true;
                    return true;
                }
            }
        }
        return false;
    }

    fn won(&self) -> bool {
        let mut win;
        // Row
        for row in self.markers.iter() {
            if row.iter().filter(|&&x| x == false).count() == 0 {
                return true;
            }
        }
        // Col
        for col in 0..self.markers[0].len() {
            win = true;
            for i in 0..self.markers.len() {
                if self.markers[i][col] == false {
                    win = false;
                    break;
                }
            }
            if win == true {
                return true;
            }
        }
        return false;
    }
    
    fn score(&self, num: u32) -> u32 {
        let mut sum = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.markers[i][j] == false {
                    sum += self.board[i][j];
                }
            }
        }
        return sum * num;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let (nums, mut boards) = convert(&contents);
    let (num, index) = mark_tiles_part2(&nums, &mut boards);
    println!("{} {}", num, index);
    println!("Winner: {}", boards[index].score(num));
}
