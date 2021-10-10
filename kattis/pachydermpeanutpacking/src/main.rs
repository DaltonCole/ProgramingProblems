use std::str::FromStr;

struct Pack {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    size: String,
}

struct Peanut {
    x: f32,
    y: f32,
    size: String,
}

impl FromStr for Pack {
    type Err = std::string::ParseError;

    fn from_str(pack_line: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = pack_line.trim().split(" ").collect();
        let x1 = v[0].parse().unwrap();
        let y1 = v[1].parse().unwrap();
        let x2 = v[2].parse().unwrap();
        let y2 = v[3].parse().unwrap();
        let size = v[4];

        Ok(Pack { x1, x2, y1, y2, size: size.to_string() })
    }
}

impl FromStr for Peanut {
    type Err = std::string::ParseError;

    fn from_str(peanut_line: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = peanut_line.trim().split(" ").collect();
        let x = v[0].parse().unwrap();
        let y = v[1].parse().unwrap();
        let size = v[2];

        Ok(Peanut { x, y, size: size.to_string() })
    }
}

fn read_num() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim()
        .parse()
        .unwrap()
}

fn get_packs(lines: u32) -> Vec<Pack> {
    let mut v: Vec<Pack> = Vec::new();
    for _ in 0..lines {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        v.push(
            buffer.trim().parse().unwrap()
        );
    }
    v
}

fn get_peanuts(lines: u32) -> Vec<Peanut> {
    let mut v: Vec<Peanut> = Vec::new();
    for _ in 0..lines {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        v.push(
            buffer.trim().parse().unwrap()
        )
    }
    v
}

fn peanut_in_pack(pack: &Pack, peanut: &Peanut) -> bool {
    if pack.x1 <= peanut.x && peanut.x <= pack.x2 &&
        pack.y1 <= peanut.y && peanut.y <= pack.y2 {
            return true;
    }
    false
}

fn peanut_placement(packs: &Vec<Pack>, peanuts: &Vec<Peanut>) {
    for peanut in peanuts {
        let mut found = false;
        for pack in packs {
            if peanut_in_pack(&pack, &peanut) {
                if pack.size == peanut.size {
                    println!("{} correct", pack.size);
                } else {
                    println!("{} {}", peanut.size, pack.size);
                }
                found = true;
                break;
            }
        }
        if found == false {
            println!("{} floor", peanut.size);
        }
    }
}

fn main() {
    loop {
        let lines = read_num();
        if lines == 0 {
            break;
        }
        let packs = get_packs(lines);

        let peanuts_count = read_num();
        let peanuts = get_peanuts(peanuts_count);

        peanut_placement(&packs, &peanuts);
        println!();
    }
}
