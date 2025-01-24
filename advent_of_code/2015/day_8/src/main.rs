use std::io;

fn main() {
    let mut total_chars = 0;
    let mut total_strings = 0;

    for line in io::stdin().lines() {
        let data = line.unwrap();
        let char_count = data.len();
        let mut escaped_count = 0;
        let mut string_count = 0;
        println!("{data} -- {}", char_count);

        let character: Vec<char> = data.chars().collect();

        if char_count > 2 {
            for (two_back, letter) in character[2..character.len() - 1].iter().enumerate() {
                let pair = format!("{}{}", character[two_back + 1], letter);

                if pair.eq("\\\"") || pair.eq("\\\\") {
                    escaped_count += 1;
                } else if pair.eq("\\x") {
                    escaped_count += 3; // for hex
                }
            }
            string_count = char_count - escaped_count - 2;
        }
        println!("{data} - Char count: {char_count} - Escaped count: {escaped_count} - Diff (char - eascped - 2) {string_count}");
        total_chars += char_count;
        total_strings += string_count;
    }
    println!(
        "Chars: {total_chars}, Strings: {total_strings}, Diff: {}",
        total_chars - total_strings
    );
}
