use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub s: i32,
}

impl Point {
    fn move_bee(&self, a: i32, b: i32, c: i32) -> Point {
        Point{
            x: self.x + a,
            y: self.y + b,
            s: self.s + c,
        }
    }
}

fn get_num() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_answer(num: usize) -> u32 {
    let answer = [0 , 0, 6, 12, 90, 360, 2040, 10080, 54810, 290640, 1588356, 8676360, 47977776, 266378112, 1488801600];
    
    answer[num]
}

/// ```
/// for num in 2..14 {
///     assert_eq!(get_answer(num as usize),
///         dynamic(
///             Point{x: 0, y: 0, s: num as i32},
///             &mut HashMap::new()
///         )
///     )
/// }
/// ```
fn dynamic(point: Point, points: &mut HashMap<Point, u32>) -> u32 {
    if point.s == 0 {
        if point.x == 0 && point.y == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(n) = points.get(&point) {
        return *n;
    }

    let mut paths = 0;
    paths += dynamic(point.move_bee(-1, 0, -1), points);
    paths += dynamic(point.move_bee(0, -1, -1), points);
    paths += dynamic(point.move_bee(-1, -1, -1), points);
    paths += dynamic(point.move_bee(1, 0, -1), points);
    paths += dynamic(point.move_bee(0, 1, -1), points);
    paths += dynamic(point.move_bee(1, 1, -1), points);

    points.insert(point, paths);

    paths
}

fn main() {
    for _ in 0..get_num() {
        let num = get_num();
        //println!("{}", get_answer(num));
        println!("{}", dynamic(
                Point{x: 0, y: 0, s: num as i32},
                &mut HashMap::new()
        ));
    }
}
