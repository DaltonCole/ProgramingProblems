fn get_num() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_line() -> (char, f64) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<&str> = line.trim().split(" ").collect();
    (v[0].chars().next().unwrap(), v[1].parse().unwrap())
}

fn update_position(x: f64, y: f64, degrees: f64, magnitude: f64) -> (f64, f64) {
    let new_x = x + (magnitude * (degrees.to_radians()).cos());
    let new_y = y + (magnitude * (degrees.to_radians()).sin());
    (new_x, new_y)
}

fn main() {
    let cases = get_num();
    for _ in 0..cases {
        let num_lines = get_num();
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut degrees: f64 = 0.0;

        for _ in 0..num_lines {
            let (action, mut magnitude) = get_line();
            // Adjust magnitude direction
            if action == 'r' || action == 'b' {
                magnitude *= -1.0;
            }
            // Adjust angle
            if action == 'l' || action == 'r' {
                degrees = (degrees + magnitude) % 360.0;
            }
            // Go direction
            else {
                let tmp = update_position(x, y, degrees, magnitude);
                x = tmp.0;
                y = tmp.1;
            }
        }
        let distance = (x.powi(2) + y.powi(2)).sqrt().round();
        println!("{}", distance);
    }
}
