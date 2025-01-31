use std::io;

fn new_string(line: &str) -> String {
    let mut count = 0;
    let mut result = String::new();

    let mut current_char = None;
    for character in line.chars() {
        match current_char {
            None => {
                current_char = Some(character);
                count = 1;
            }
            Some(old_char) => {
                if old_char.eq(&character) {
                    count += 1;
                } else {
                    result.push_str(format!("{count}{old_char}").as_str());
                    current_char = Some(character);
                    count = 1;
                }
            }
        }
    }
    // This is here to pick up the last count and char
    result.push_str(format!("{count}{}", current_char.unwrap()).as_str());
    result
}

fn main() {
    for line in io::stdin().lines() {
        let mut data = line.unwrap();
        println!("Start: {data}");

        for round in 1..=50 {
            let prev_data = data.clone();
            data = new_string(&data);
            println!("({round}): {prev_data} -->  {data}");
        }
        println!("The final result has {} digits in it.", data.len());
        println!("\n\n");
    }
}
