struct Solution;

enum AmountOverlap {
    Perfect(Rect),
    Partial(Rect),
    None,
}

struct Rect {
    x1: usize, // Top left
    y1: usize,
    x2: usize, // Bottom right
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2: usize) -> Rect {
        Rect { x1, y1, x2 }
    }
    pub fn area(&self, y2: usize) -> usize {
        (y2 + 1 - self.y1) * (self.x2 + 1 - self.x1)
    }

    pub fn overlapping(&self, x1: usize, x2: usize) -> AmountOverlap {
        // Perfect overlap
        if self.x1 == x1 && self.x2 == x2 {
            return AmountOverlap::Perfect(Rect::new(self.x1, self.y1, self.x2));
        }
        // Inside - New row is inside the old row
        else if self.x1 <= x1 && x2 <= self.x2 {
            return AmountOverlap::Perfect(Rect::new(x1, self.y1, x2));
        }
        // Shifted left
        else if x1 <= self.x1 && self.x1 <= x2 && x2 <= self.x2 {
            // Keep what we can of the old
            return AmountOverlap::Partial(Rect::new(self.x1, self.y1, x2));
        }
        // Shifted right
        else if self.x1 <= x1 && x1 <= self.x2 && self.x2 <= x2 {
            // Keep what we can of the old
            return AmountOverlap::Partial(Rect::new(x1, self.y1, self.x2));
        }
        // Outside - New row is wider and fully encompasses
        else if x1 <= self.x1 && self.x2 <= x2 {
            // Keep what we can of the old
            return AmountOverlap::Partial(Rect::new(self.x1, self.y1, self.x2));
        }

        AmountOverlap::None
    }
}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_area = 0;
        let mut candidates: Vec<Rect> = Vec::new();

        for (y, row) in matrix.iter().enumerate() {
            let mut new_candidates: Vec<Rect> = Vec::new();
            // Where a series of 1s started in a row
            let mut starting_one_x = None;
            // Keep track of each series of 1s in a row in (x1, x2) format
            let mut ones_section = Vec::new();
            // Go through each element in a column
            for (x, &ele) in row.iter().enumerate() {
                // Find were this series of 1s start on this row
                if ele == '0' {
                    // Add to ones_selection if we encountered a 0 and had already found a 1
                    if let Some(starting_x) = starting_one_x {
                        ones_section.push((starting_x, x - 1));
                    }
                    starting_one_x = None;
                } else {
                    if let None = starting_one_x {
                        starting_one_x = Some(x)
                    }
                }
            }
            // Add last starting_one_x if the row ended on a 1
            if let Some(starting_x) = starting_one_x {
                ones_section.push((starting_x, row.len() - 1));
            }

            // --- Add candidates --- //
            for section in ones_section {
                let mut perfect_overlap = false;

                for candidate in candidates.iter() {
                    match candidate.overlapping(section.0, section.1) {
                        AmountOverlap::Perfect(new_candidate) => {
                            new_candidates.push(new_candidate);
                            perfect_overlap = true;
                        }
                        AmountOverlap::Partial(new_candidate) => {
                            new_candidates.push(new_candidate);
                        }
                        AmountOverlap::None => {}
                    }
                }

                if !perfect_overlap {
                    new_candidates.push(Rect::new(section.0, y, section.1));
                }
            }
            // Flip candidates and new_candidates
            candidates = new_candidates;

            // Find all of the areas
            for candidate in candidates.iter() {
                max_area = std::cmp::max(max_area, candidate.area(y))
            }
        }

        max_area as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            6,
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            6,
            Solution::maximal_rectangle(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        )
    }

    #[test]
    fn test3() {
        assert_eq!(1, Solution::maximal_rectangle(vec![vec!['1']]))
    }

    #[test]
    fn test4() {
        assert_eq!(0, Solution::maximal_rectangle(vec![vec!['0'],]))
    }
}
