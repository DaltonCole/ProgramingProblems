struct Time {
    start: u32,
    end: u32,
}

impl Time {
    fn new(start: u32, duration: u32) -> Time {
        Time{
            start,
            end: start + duration,
        }
    }

    fn includes(&self, interested: &Time) -> bool {
        (interested.start <= self.start && self.start < interested.end) ||
            interested.start < self.end && self.end <= interested.end ||
            (self.start < interested.start && self.end > interested.end)
    }
}

fn get_input() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    loop {
        let tmp = get_input();
        let (num_calls, num_interested) = (tmp[0], tmp[1]);
        if num_calls == 0 && num_interested == 0 {
            break;
        }
        let mut times = Vec::<Time>::new();
        for _ in 0..num_calls {
            let tmp = get_input();
            times.push(Time::new(tmp[2], tmp[3]));
        }
        for _ in 0..num_interested {
            let tmp = get_input();
            let interested = Time::new(tmp[0], tmp[1]);
            let mut count = 0;
            for time in &times {
                if time.includes(&interested) {
                    count += 1;
                }
            }
            println!("{}", count);
        }
    }
}
