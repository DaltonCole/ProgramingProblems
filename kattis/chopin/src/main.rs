use std::str::FromStr;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Note {
    note: char,
    accidental: Option<char>,
    tonal: String,
}

impl FromStr for Note {
    type Err = std::string::ParseError;

    fn from_str(note_line: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = note_line.trim().split(" ").collect();
        let note = v[0].chars().nth(0).unwrap();
        let accidental;
        if v[0].len() == 2 {
            accidental = Option::Some(v[0].chars().nth(1).unwrap());
        } else {
            accidental = Option::None;
        }
        
        Ok(Note { note, accidental, tonal: v[1].to_string() })
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.accidental == None {
            return write!(f, "UNIQUE");
        } else {
            if self.accidental == Some('b') {
                if self.note != 'A' {
                    return write!(f, "{}# {}", (self.note as u8 - 1) as char, self.tonal);
                } else {
                    return write!(f, "G# {}", self.tonal);
                }
            } else {
                if self.note != 'G' {
                    return write!(f, "{}b {}", (self.note as u8 + 1) as char, self.tonal);
                } else {
                    return write!(f, "Ab {}", self.tonal);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut count = 0;
    while let Some(line) = lines.next() {
        count += 1;
        let note: Note = line.unwrap().parse().unwrap();
        println!("Case {}: {}", count, note);

    }
}
