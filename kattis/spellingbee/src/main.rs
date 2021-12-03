fn get_word() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_num() -> u32 {
    let num = get_word();
    num.parse().unwrap()
}

fn word_to_letters(word: &String) -> Vec<char> {
    word.chars().collect()
}

fn get_words() -> Vec<String> {
    let mut v = Vec::<String>::new();
    for _ in 0..get_num() {
        v.push(get_word());
    }
    v
}

fn main() {
    let main_word = get_word();
    let letters = word_to_letters(&main_word);
    let words = get_words();

    for word in words {
        let mut contains = true;
        if word.chars().count() >= 4 && word.contains(letters[0]) {
            for c in word.chars() {
                if !letters.contains(&c) {
                    contains = false;
                    break;
                }
            }
            if contains == true {
                println!("{}", word);
            }
        }
    }
}
