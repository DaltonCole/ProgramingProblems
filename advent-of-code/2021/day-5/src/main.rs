use std::cmp::{min, max};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

struct Matrix {
    matrix: HashMap<(i32, i32), i32>,
}

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn get_lines(contents: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for line in &mut contents.lines() {
        let fixed_line = line.replace(" -> ", ",");
        let parts: Vec<i32> = fixed_line.split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        lines.push(Line{
            x1: parts[0],
            y1: parts[1],
            x2: parts[2],
            y2: parts[3],
        })
    }
    lines
}

impl Matrix {
    fn new(lines: &Vec<Line>) -> Matrix {
        let mut matrix = HashMap::new();


        for line in lines {
            // Vertical
            if line.x1 == line.x2 {
                for y in min(line.y1, line.y2)..=max(line.y1, line.y2) {
                    let point = matrix.entry((line.x1, y)).or_insert(0);
                    *point += 1;
                }
            }
            // Horizontal
            else if line.y1 == line.y2 {
                for x in min(line.x1, line.x2)..=max(line.x1, line.x2) {
                    let point = matrix.entry((x, line.y1)).or_insert(0);
                    *point += 1;
                }
            }
            // Diagonal
            else {
                let x1 = min(line.x1, line.x2);
                let x2 = max(line.x1, line.x2);
                let mut y1 = if x1 == line.x1 {line.y1} else {line.y2};
                let y2 = if x2 == line.x2 {line.y2} else {line.y1};

                for x in x1..=x2 {
                    let point = matrix.entry((x, y1)).or_insert(0);
                    *point += 1;

                    if y1 <= y2 {
                        y1 += 1;
                    } else {
                        y1 -= 1;
                    }
                }
            }

        }

        Matrix{ matrix }
    }

    fn count_intersections(&self) -> i32 {
        self.matrix.iter().filter(|(_, count)| count > &&1).count() as i32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let lines = get_lines(&contents);
    let matrix = Matrix::new(&lines);
    let intersections = matrix.count_intersections();
    println!("{}", intersections);
}
