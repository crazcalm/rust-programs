use std::io;

fn main() {
    let mut total_chars = 0;
    let mut total_expanded = 0;

    for line in io::stdin().lines() {
        let data = line.unwrap();

        let mut expanded_count = 0;
        for character in data.chars() {
            match character {
                '\\' => expanded_count += 1,
                '"' => expanded_count += 1,
                _ => {}
            }
        }

        let expanded_count = data.len() + expanded_count + 2;
        println!(
            "line: {data} -- chars: {} -- expanded: {expanded_count}",
            data.len()
        );
        total_expanded += expanded_count;
        total_chars += data.len();
    }

    println!(
        "{total_expanded} - {total_chars} = {}",
        total_expanded - total_chars
    );
}
