use std::io;

fn gcd(mut u: u32, mut v: u32) -> u32 {
    let mut t;
    while u > 0 {
        if u < v {
            t = u;
            u = v;
            v = t;
        }
        u = u - v;
    }
    v
}

fn main() {
    for line in io::stdin().lines() {
        let elements: Vec<u32> = line
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u32>().expect("unable to parse string into i32"))
            .collect();

        match elements.len() {
            2 => {
                if elements[0] > 0 && elements[1] > 0 {
                    println!(
                        "gcd({}, {}) => {}",
                        elements[0],
                        elements[1],
                        gcd(elements[0], elements[1])
                    );
                }
            }
            _ => println!("The string must be two integers separated by a space"),
        }
    }
}
