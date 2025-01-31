use std::collections::HashMap;
use std::io;

struct Password {
    internal: Vec<usize>,
    number_map: HashMap<usize, String>,
    letter_map: HashMap<String, usize>,
}

impl Password {
    fn new(
        data: &str,
        number_map: HashMap<usize, String>,
        letter_map: HashMap<String, usize>,
    ) -> Self {
        let internal: Vec<usize> = data
            .chars()
            .map(|x| x.to_string())
            .map(|y| letter_map.get(&y).unwrap().clone())
            .collect();
        Self {
            internal,
            number_map,
            letter_map,
        }
    }

    fn to_string(&self) -> String {
        self.internal
            .iter()
            .map(|x| self.number_map.get(x).unwrap())
            .map(|y| y.clone())
            .collect()
    }

    fn increment(&mut self) {
        let mut index = self.internal.len() - 1;

        loop {
            match self.internal[index] {
                25 => {
                    self.internal[index] = 0;
                    if index == 0 {
                        break;
                    } else {
                        index -= 1;
                    }
                }
                _ => {
                    self.internal[index] += 1;
                    break;
                }
            }
        }
    }
}

fn validate(funcs: Vec<fn(&str) -> bool>, input: String) -> bool {
    let mut result = true;

    for func in funcs.iter() {
        match func(&input) {
            false => {
                result = false;
                break;
            }
            true => {}
        }
    }
    result
}

fn contains_unwanted_letters(data: &str) -> bool {
    let mut result = true;

    for letter in data.chars() {
        match letter {
            'o' | 'i' | 'l' => {
                result = false;
                break;
            }
            _ => {}
        }
    }
    result
}

fn contains_two_pairs(data: &str) -> bool {
    let mut pairs = Vec::new();

    let list: Vec<char> = data.chars().collect();

    let mut index = 1;
    while index < list.len() {
        let prev_char = list[index - 1];
        let curr_char = list[index];

        match prev_char.eq(&curr_char) {
            true => {
                if !pairs.contains(&format!("{prev_char}{curr_char}")) {
                    pairs.push(format!("{prev_char}{curr_char}"));
                    index += 1;
                }
            }
            false => {}
        }

        index += 1;
    }

    pairs.len() >= 2
}

fn contains_seq_letters(data: &str) -> bool {
    let letter_map: HashMap<String, usize> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|x| x.to_string())
        .enumerate()
        .map(|(num, letter)| (letter, num))
        .collect();

    let mut result = false;

    let list: Vec<String> = data.chars().map(|x| x.to_string()).collect();
    let mut index = 2;

    while index < list.len() {
        let num_1 = letter_map.get(&list[index - 2]).unwrap();
        let num_2 = letter_map.get(&list[index - 1]).unwrap();
        let num_3 = letter_map.get(&list[index]).unwrap();

        //println!("{}, {}, {}", num_1, num_2, num_3);

        if num_1 + 1 == *num_2 && num_2 + 1 == *num_3 {
            result = true;
            break;
        }
        index += 1;
    }
    //println!("seq: {result}");
    result
}

fn main() {
    let mut letter_to_number = HashMap::new();
    let mut number_to_letter = HashMap::new();

    for (index, letter) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
        letter_to_number.insert(index.clone(), letter.to_string());
        number_to_letter.insert(letter.to_string(), index);
    }

    for line in io::stdin().lines() {
        let data = line.unwrap();

        let mut password = Password::new(
            &data.to_string(),
            letter_to_number.clone(),
            number_to_letter.clone(),
        );

        let mut result = password.to_string();
        let checks = vec![
            contains_seq_letters,
            contains_unwanted_letters,
            contains_two_pairs,
        ];

        while !validate(checks.clone(), result.clone()) {
            password.increment();
            result = password.to_string();
            //println!("{result}");
        }

        println!("{result}");
    }
}
