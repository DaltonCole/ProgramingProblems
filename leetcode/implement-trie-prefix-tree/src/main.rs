use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    letters: HashMap<char, Box<Trie>>,
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            letters: HashMap::new(),
            end: false,
        }
    }

    fn insert(&mut self, word: String) {
        self.insert_str(&word)
    }

    fn insert_str(&mut self, word: &str) {
        let mut current_letter = self;

        for letter in word.chars() {
            current_letter = current_letter
                .letters
                .entry(letter)
                .or_insert(Box::new(Trie::new()));
        }

        current_letter.end = true;
    }

    fn search(&self, word: String) -> bool {
        return self.search_str(&word, false);
    }

    fn starts_with(&self, prefix: String) -> bool {
        return self.search_str(&prefix, true);
    }

    fn search_str(&self, word: &str, starts_with: bool) -> bool {
        let mut current_letter = self;

        for letter in word.chars() {
            match current_letter.letters.get(&letter) {
                Some(next_letter) => {
                    current_letter = next_letter;
                }
                None => {
                    return false;
                }
            }
        }

        current_letter.end || starts_with
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
