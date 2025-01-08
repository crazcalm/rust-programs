use std::collections::{HashMap, HashSet};
use std::io;

fn is_digit(data: &str) -> bool {
    let mut result = true;
    for character in data.chars() {
        if !character.is_digit(10) {
            result = false;
            break;
        }
    }
    result
}

fn compliment(data: Vec<&str>, map: &mut HashMap<String, u16>) {
    let a = data[1];

    let digit = match is_digit(a) {
        true => a.parse::<u16>().unwrap(),
        false => map.get(a).unwrap().clone(),
    };

    map.insert(data[3].to_string(), !digit);
}

fn assign_variable(data: Vec<&str>, map: &mut HashMap<String, u16>) {
    let a = data[0];
    let b = data[2];

    match is_digit(a) {
        true => {
            let digit = a.parse::<u16>().unwrap();
            map.insert(b.to_string(), digit);
        }
        false => {
            let digit = map.get(a).unwrap().clone();
            map.insert(b.to_string(), digit);
        }
    }
}

fn and(data: Vec<&str>, map: &mut HashMap<String, u16>) {
    let a = data[0];
    let b = data[2];

    let digit_1 = match is_digit(a) {
        true => a.parse::<u16>().unwrap(),
        false => map.get(a).unwrap().clone(),
    };
    let digit_2 = match is_digit(b) {
        true => b.parse::<u16>().unwrap(),
        false => map.get(b).unwrap().clone(),
    };

    map.insert(data[4].to_string(), digit_1 & digit_2);
}

fn or(data: Vec<&str>, map: &mut HashMap<String, u16>) {
    let a = data[0];
    let b = data[2];

    let digit_1 = match is_digit(a) {
        true => a.parse::<u16>().unwrap(),
        false => map.get(a).unwrap().clone(),
    };
    let digit_2 = match is_digit(b) {
        true => b.parse::<u16>().unwrap(),
        false => map.get(b).unwrap().clone(),
    };

    map.insert(data[4].to_string(), digit_1 | digit_2);
}

fn left_shift(data: Vec<&str>, map: &mut HashMap<String, u16>) {
    let a = data[0];
    let b = data[2].parse::<u16>().unwrap();

    let digit_1 = match is_digit(a) {
        true => a.parse::<u16>().unwrap(),
        false => map.get(a).unwrap().clone(),
    };

    map.insert(data[4].to_string(), digit_1 << b);
}

fn right_shift(data: Vec<&str>, map: &mut HashMap<String, u16>) {
    let a = data[0];
    let b = data[2].parse::<u16>().unwrap();

    let digit_1 = match is_digit(a) {
        true => a.parse::<u16>().unwrap(),
        false => map.get(a).unwrap().clone(),
    };

    map.insert(data[4].to_string(), digit_1 >> b);
}

fn process_line(command: &str, map: &mut HashMap<String, u16>) {
    let data: Vec<&str> = command.split(" ").collect();

    println!("data: {:?} ", data);

    if data.len() == 3 {
        assign_variable(data, map);
    } else if data.len() == 4 {
        compliment(data, map);
    } else if data.len() == 5 {
        match data[1] {
            "AND" => and(data, map),
            "OR" => or(data, map),
            "LSHIFT" => left_shift(data, map),
            "RSHIFT" => right_shift(data, map),
            _ => {}
        }
    }
}

fn needed_vars_set(command: &str, map: &HashMap<String, u16>) -> bool {
    let mut result = true;

    let ignore_list = HashSet::from(["AND", "OR", "LSHIFT", "RSHIFT", "NOT"]);

    for part in command.split(" ") {
        if part.eq("->") {
            break;
        }
        if is_digit(part) {
            continue;
        }

        if ignore_list.contains(part) {
            continue;
        }

        if map.get(part).is_none() {
            result = false;
            break;
        }
    }
    result
}

fn main() {
    let mut old_commands = Vec::new();
    let mut variables: HashMap<String, u16> = HashMap::new();

    for line in io::stdin().lines() {
        let command = line.unwrap();

        if needed_vars_set(&command, &variables) {
            process_line(&command, &mut variables);
        } else {
            old_commands.push(command);
        }
    }

    while old_commands.len() > 0 {
        let mut other_commands = Vec::new();
        for command in old_commands.iter() {
            //println!("{command}, {:#?}", variables);

            if needed_vars_set(command, &variables) {
                process_line(command, &mut variables);
            } else {
                other_commands.push(command.clone());
            }
        }
        old_commands = other_commands;
    }

    println!("{:?}", variables.get("a"));
}
