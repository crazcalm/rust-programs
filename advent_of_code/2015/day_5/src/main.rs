use std::collections::HashSet;
use std::io;

fn main() {
    let mut nice_string = 0;

    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let bad_pairs = HashSet::from(["ab", "cd", "pq", "xy"]);

    for line in io::stdin().lines() {
        let mut vowel_count = 0;
        let mut double_char_count = 0;
        let mut bad_pair_count = 0;
        let mut prev_char = None;

        for (index, character) in line.unwrap().chars().enumerate() {
            if vowels.contains(&character) {
                vowel_count += 1;
            }

            match prev_char {
                None => prev_char = Some(character),
                Some(prev) => {
                    if prev.eq(&character) {
                        double_char_count += 1;
                    }

                    if bad_pairs.contains(format!("{}{}", prev, character).as_str()) {
                        bad_pair_count += 1;
                    }

                    prev_char = Some(character)
                }
            }
        }

        if bad_pair_count == 0 && vowel_count >= 3 && double_char_count >= 1 {
            nice_string += 1;
        }

        println!(
            "nice string count: {} ---- vowels: {}, doubles: {}, bad pairs: {}",
            nice_string, vowel_count, double_char_count, bad_pair_count
        );
    }
}
