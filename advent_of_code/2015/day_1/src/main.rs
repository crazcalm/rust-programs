use std::io;

fn main() {
    let mut floor = 0;

    for line in io::stdin().lines() {
        for character in line.unwrap().chars() {
            if character.eq(&'(') {
                floor += 1;
            } else if character.eq(&')') {
                floor -= 1;
            }
        }
    }
    println!("Floor {}", floor);
}
