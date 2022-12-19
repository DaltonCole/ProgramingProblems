struct Solution;

#[derive(Debug)]
struct Group {
    next: i32,
    needed_elements: i32,
}

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        // Low hanging fruit
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        if group_size == 1 {
            return true;
        }

        // Sort hand
        hand.sort();

        let mut groups = Vec::new();

        for card in hand {
            if groups.len() == 0 {
                groups.push(Group {
                    next: card + 1,
                    needed_elements: group_size - 1,
                });
            } else {
                let mut used = false;
                // Go through each group to see if card is useful
                for i in 0..groups.len() {
                    if groups[i].next == card {
                        groups[i].next += 1;
                        groups[i].needed_elements -= 1;
                        used = true;

                        if groups[i].needed_elements == 0 {
                            groups.remove(i);
                        }
                        break;
                    }
                }
                if used == false {
                    groups.push(Group {
                        next: card + 1,
                        needed_elements: group_size - 1,
                    });
                }
            }
            #[cfg(test)]
            println!("--- {} ---", card);
            #[cfg(test)]
            println!("{:#?}", groups);
        }

        #[cfg(test)]
        println!("{:#?}", groups);

        groups.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_n_straight_hand(
            vec![1, 2, 3, 4, 5, 6, 7, 9],
            4
        ));
    }
}
