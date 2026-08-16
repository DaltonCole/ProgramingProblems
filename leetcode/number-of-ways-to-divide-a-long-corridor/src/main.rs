struct Solution;

impl Solution {
    pub fn number_of_ways(mut corridor: String) -> i32 {
        let s_count = corridor.chars().filter(|&c| c == 'S').count();

        if s_count == 0 || s_count % 2 == 1 {
            return 0;
        }

        let mod_value = 10_u64.pow(9) + 7;

        let mut ans = 1;
        let mut plants = 1;
        let mut chairs = 0;

        while let Some(c) = corridor.pop() {
            if c == 'S' {
                if chairs == 2 {
                    ans = ((ans as u64 * plants as u64) % mod_value) as i32;
                    plants = 1;
                    chairs = 0;
                }
                chairs += 1;
            } else {
                if chairs == 2 {
                    plants += 1;
                }
            }
        }
        //ans *= plants;

        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::number_of_ways("SSPPSPS".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(1, Solution::number_of_ways("PPSPSP".to_string()));
    }

    #[test]
    fn test3() {
        assert_eq!(0, Solution::number_of_ways("S".to_string()));
    }

    #[test]
    fn test4() {
        assert_eq!(9, Solution::number_of_ways("SSPPSPSSSPPSPS".to_string()));
    }

    #[test]
    fn test5() {
        assert_eq!(919999993, Solution::number_of_ways("PPPPPSPPSPPSPPPSPPPPSPPPPSPPPPSPPSPPPSPSPPPSPSPPPSPSPPPSPSPPPPSPPPPSPPPSPPSPPPPSPSPPPPSPSPPPPSPSPPPSPPSPPPPSPSPSS".to_string()));
    }

    #[test]
    fn test6() {
        assert_eq!(18335643, Solution::number_of_ways("PPPPPPPSPPPSPPPPSPPPSPPPPPSPPPSPPSPPSPPPPPSPSPPPPPSPPSPPPPPSPPSPPSPPPSPPPPSPPPPSPPPPPSPSPPPPSPSPPPSPPPPSPPPPPSPSPPSPPPPSPPSPPSPPSPPPSPPSPSPPSSSS".to_string()));
    }
}
