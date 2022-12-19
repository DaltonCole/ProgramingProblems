#[derive(Debug)]
struct Trie {
    letter: Option<char>,
    end: bool,
    next_letters: Vec<Box<Trie>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            letter: None,
            end: false,
            next_letters: Vec::new(),
        }
    }

    fn new_letter(letter: char) -> Self {
        Trie {
            letter: Some(letter),
            end: false,
            next_letters: Vec::new(),
        }
    }

    fn insert(&mut self, word: String) {
        self.insert_str(&word)
    }

    fn insert_str(&mut self, word: &str) {
        if word.len() == 0 {
            self.end = true;
            return;
        }

        let first_letter = word.chars().next().unwrap();

        // Search for first letter in next_letters
        for next_letter in &mut self.next_letters {
            if let Some(c) = next_letter.letter {
                // Found letter, go down that path with word - first_letter
                if c == first_letter {
                    next_letter.insert_str(&word[1..]);
                    return;
                }
            }
        }
        // First letter does not exist in next letters
        self.next_letters
            .push(Box::new(Self::new_letter(first_letter)));
        self.next_letters.last_mut().unwrap().insert_str(&word[1..])
    }

    fn search(&self, word: String) -> bool {
        return self.search_str(&word);
    }

    fn search_str(&self, word: &str) -> bool {
        if word.len() == 0 {
            return self.end;
        }

        let first_letter = word.chars().next().unwrap();
        for next_letter in &self.next_letters {
            if let Some(c) = next_letter.letter {
                if c == first_letter {
                    return next_letter.search_str(&word[1..]);
                }
            }
        }

        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        return self.starts_with_str(&prefix);
    }
    fn starts_with_str(&self, prefix: &str) -> bool {
        if prefix.len() == 0 {
            return true;
        }

        let first_letter = prefix.chars().next().unwrap();
        for next_letter in &self.next_letters {
            if let Some(c) = next_letter.letter {
                if c == first_letter {
                    return next_letter.starts_with_str(&prefix[1..]);
                }
            }
        }

        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut s = Trie::new();
        s.insert("apple".to_string());
        //println!("{s:#?}");
        assert!(s.search("apple".to_string()));
        assert!(!s.search("app".to_string()));
        assert!(s.starts_with("app".to_string()));
        s.insert("app".to_string());
        assert!(s.search("app".to_string()));
        assert!(!s.starts_with("appi".to_string()));
        assert!(!s.starts_with("zzzzzzzzzzappi".to_string()));
        assert!(!s.search("eiojrwklenlsdfsdji98s".to_string()));
    }
}
