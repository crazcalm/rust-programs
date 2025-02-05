use std::io;

fn part_1(line: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut number = String::new();
    for character in line.chars() {
        if character.eq(&'-') || character.is_digit(10) {
            number.push(character);
        } else {
            match number.is_empty() {
                true => {}
                false => {
                    println!("number to parse: {number}");
                    sum += number.parse::<i32>().unwrap().clone();
                    number = String::new();
                }
            }
        }
    }
    sum
}

fn red_zero(line: &str) -> bool {
    let mut stack = Vec::new();
    let mut red = String::new();
    let mut result = false;

    for character in line.chars() {
        match character {
            '{' => {
                stack.push('{');
            }
            '[' => {
                stack.push('[');
            }
            '}' => {
                if !stack.is_empty() {
                    stack.pop();
                }
            }
            ']' => {
                if !stack.is_empty() {
                    stack.pop();
                }
            }
            'r' => {
                if red.is_empty() {
                    red.push('r');
                } else {
                    red = String::new();
                }
            }
            'e' => {
                if red.eq("r") {
                    red.push('e');
                } else {
                    red = String::new();
                }
            }
            'd' => {
                if red.eq("re") {
                    red.push('d');

                    println!("RED");

                    if !stack.is_empty() {
                        let bracket = stack.pop().unwrap();
                        println!("bracket: {bracket}");
                        if bracket.eq(&'{') {
                            result = true;
                            break;
                        } else {
                            stack.push(bracket);
                        }
                    }
                }
            }
            _ => {
                if !red.is_empty() {
                    red = String::new();
                }
            }
        }
    }
    result
}

fn sum_string(data: &str) -> i32 {
    match red_zero(data) {
        true => 0,
        false => part_1(data),
    }
}

fn test(data: &Vec<char>, start: usize) -> i32 {
    let mut sum = 0;
    let mut temp = "{".to_string();

    for index in start..data.len() {
        match data[index] {
            '{' => {
                sum += test(&data, index + 1);
            }
            '}' => {
                temp.push('}');
                println!("temp: {temp}");
                sum += sum_string(&temp)
            }
            _ => temp.push(data[index].clone()),
        }
    }
    sum
}

fn test2(data: &mut Vec<char>, start: usize) {
    let mut sum = 0;
    let mut temp = "{".to_string();
    println!("data: {:?}", data.len());
    while data.len() > start {
        match data[start] {
            '{' => {
                data.remove(start);
                test2(data, start + 1);
            }
            '}' => {
                temp.push(data.remove(start));
                println!("temp: {temp}");
                println!("temp: {temp}: sum: {}", sum_string(&temp));
                sum += sum_string(&temp);
            }
            _ => temp.push(data.remove(start)),
        }
    }
    println!("Last sum: {sum}");
}

fn test3(data: &Vec<char>, start: usize) {
    let mut levels_stack = vec!["{".to_string(); 30];
    let mut level = 1;

    for index in start..data.len() {
        match data[index] {
            '{' => {
                level += 1;
                levels_stack[level].push('{');
            }
            '}' => {
                levels_stack[level].push('}');
                level -= 1;
            }
            _ => levels_stack[level].push(data[index].clone()),
        }
    }

    for data in levels_stack.iter() {
        println!("data: {data}\n\n");
    }

    /*
    for data in levels_stack.iter() {
        let mut count = 0;
        for c in data.chars() {
            if c.eq(&'}') {
                count += 1;
            }
        }
        if count > 1 {
            let hi: Vec<char> = data.chars().collect();
            test3(&hi, 1);
        }
    }*/
}

fn main() {
    for line in io::stdin().lines() {
        let mut line: Vec<char> = line.unwrap().chars().collect();
        println!("Sum: {:?}", test3(&line, 1));
    }
}
