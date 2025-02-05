use std::io;

fn main() {
    for line in io::stdin().lines() {
        let mut number = String::new();
        let mut sum: i32 = 0;
        for character in line.unwrap().chars() {
            if character.eq(&'-') || character.is_digit(10) {
                number.push(character);
            } else {
                match number.is_empty() {
                    true => {}
                    false => {
                        sum += number.parse::<i32>().unwrap().clone();
                        number = String::new();
                    }
                }
            }
        }

        println!("sum: {sum}");
    }
}
