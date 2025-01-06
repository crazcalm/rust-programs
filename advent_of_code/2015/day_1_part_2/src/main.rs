use std::io;

fn main() {
    let mut floor = 0;
    let mut position = 0;

    for line in io::stdin().lines() {
        for character in line.unwrap().chars() {
            position += 1;
            if character.eq(&'(') {
                floor += 1;
            } else if character.eq(&')') {
                floor -= 1;
            }

            if floor == -1 {
                break;
            }
        }
    }
    println!(
        "Santa has reached the basement floor at position {}",
        position
    );
}
