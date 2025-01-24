use std::io;

fn main() {
    let mut total_chars = 0;
    let mut total_strings = 0;

    for line in io::stdin().lines() {
        let data = line.unwrap();
        let char_count = data.len();
        let mut escaped_count = 0;

        let characters: Vec<char> = data.chars().collect();

        let mut index = 2;
        while index < characters.len() - 1 {
            let prev = index - 1;

            let pair = format!("{}{}", characters[prev], characters[index]);
            if pair.eq("\\\"") || pair.eq("\\\\") {
                escaped_count += 1;
                index += 1 // skip over 1 to prevent double counting
            } else if pair.eq("\\x") {
                escaped_count += 3; // for hex
                index += 3 // skip over 3 to prevent double counting
            }

            index += 1
        }
        let string_count = char_count - escaped_count - 2;
        println!("{data} - Char count: {char_count} - Escaped count: {escaped_count} - Diff (char - eascped - 2 = string count) {string_count}");
        total_chars += char_count;
        total_strings += string_count;
    }
    println!(
        "Chars: {total_chars}, Strings: {total_strings}, Diff: {}",
        total_chars - total_strings
    );
}
