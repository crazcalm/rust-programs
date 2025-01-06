use std::collections::HashSet;
use std::io;

fn main() {
    let mut up_down = 0;
    let mut left_right = 0;
    let mut set = HashSet::new();

    set.insert((0, 0));

    for line in io::stdin().lines() {
        for character in line.unwrap().chars() {
            if character.eq(&'^') {
                up_down += 1;
            } else if character.eq(&'v') {
                up_down -= 1;
            } else if character.eq(&'<') {
                left_right -= 1;
            } else if character.eq(&'>') {
                left_right += 1;
            }

            set.insert((up_down, left_right));
        }
    }

    println!("Santa delivered presents to {} homes", set.len());
}
