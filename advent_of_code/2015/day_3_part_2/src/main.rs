use std::collections::HashSet;
use std::io;

fn update_coordinates(
    direction: &char,
    x: &mut isize,
    y: &mut isize,
    set: &mut HashSet<(isize, isize)>,
) {
    if direction.eq(&'^') {
        *x += 1;
    } else if direction.eq(&'v') {
        *x -= 1;
    } else if direction.eq(&'<') {
        *y -= 1;
    } else if direction.eq(&'>') {
        *y += 1;
    }

    set.insert((*x, *y));
}

fn main() {
    let mut santa_up_down = 0;
    let mut santa_left_right = 0;
    let mut robot_up_down = 0;
    let mut robot_left_right = 0;

    let mut set = HashSet::new();

    set.insert((0, 0));

    for line in io::stdin().lines() {
        for (index, character) in line.unwrap().chars().enumerate() {
            if index % 2 == 0 {
                update_coordinates(
                    &character,
                    &mut santa_up_down,
                    &mut santa_left_right,
                    &mut set,
                );
            } else {
                update_coordinates(
                    &character,
                    &mut robot_up_down,
                    &mut robot_left_right,
                    &mut set,
                );
            }
        }
    }

    println!("Santa delivered presents to {} homes", set.len());
}
