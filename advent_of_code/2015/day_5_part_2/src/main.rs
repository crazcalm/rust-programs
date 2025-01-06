use std::collections::HashMap;
use std::io;

fn main() {
    let mut nice_words_count = 0;

    for line in io::stdin().lines() {
        let vec_chars: Vec<char> = line.unwrap().chars().collect();

        let mut index = 1;
        let mut doubles_count = HashMap::new();
        let mut char_repeat_count = 0;

        for character in vec_chars[1..].iter() {
            if index >= 2 {
                if vec_chars[index - 2].eq(character) {
                    char_repeat_count += 1;
                }
            }

            match index {
                1 => {
                    doubles_count
                        .entry(format!("{}{}", vec_chars[index - 1], vec_chars[index]))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                2 => {
                    if vec_chars[index - 1].eq(character) && vec_chars[index - 2].eq(character) {
                    } else {
                        doubles_count
                            .entry(format!("{}{}", vec_chars[index - 1], vec_chars[index]))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
                x if x >= 3 => {
                    if vec_chars[index - 1].eq(character) && vec_chars[index - 2].eq(character) {
                        if vec_chars[index - 3].eq(character) {
                            doubles_count
                                .entry(format!("{}{}", vec_chars[index - 1], vec_chars[index]))
                                .and_modify(|count| *count += 1)
                                .or_insert(1);
                        }
                    } else {
                        doubles_count
                            .entry(format!("{}{}", vec_chars[index - 1], vec_chars[index]))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
                _ => {}
            }

            index += 1;
        }

        if char_repeat_count >= 1 && doubles_count.values().any(|&x| x > 1) {
            nice_words_count += 1;
        }

        println!(
            "nice_words_count: {}, char repeat count: {}, doubles count: {:?}",
            nice_words_count, char_repeat_count, doubles_count
        );
    }
}
