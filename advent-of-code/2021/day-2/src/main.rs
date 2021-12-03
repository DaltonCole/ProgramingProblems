use std::env;
use std::fs;
use std::io;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn convert_contents(contents: &str) -> Vec<(String, i32)> {
    let lines: Vec<String> = contents.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.to_string())
        .collect();
    let mut v = Vec::<(String, i32)>::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        v.push((parts[0].to_string(), parts[1].parse().unwrap()));
    }
    v
}

fn follow_inst_part_1(instructions: &Vec<(String, i32)>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    for (inst, num) in instructions {
        match inst.as_str() {
            "forward" => x += num,
            "down" => y += num,
            "up" => y -= num,
            _ => (),
        }

    }
    (x, y)
}

fn follow_inst_part_2(instructions: &Vec<(String, i32)>) -> (i32, i32) {
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (inst, num) in instructions {
        match inst.as_str() {
            "forward" => {
                h_pos += num;
                depth += (aim * num);
            },
            "down" => aim += num,
            "up" => aim -= num,
            _ => (),
        }

    }
    (h_pos, depth)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let instru = convert_contents(&contents);
    let (x, y) = follow_inst_part_2(&instru);
    println!("{}", x * y);
}
