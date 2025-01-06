use std::io;

use md5;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^0{6}[a-f1-9]").unwrap();

    for line in io::stdin().lines() {
        let mut found = None;

        let key = line.as_ref().unwrap();
        print!("{key}");

        for num in 0..=2000000000 {
            let digest = md5::compute(format!("{}{}", line.as_ref().unwrap(), num));

            if re.is_match(format!("{:x}", digest).as_str()) {
                println!(" -- {:?}", digest);
                found = Some(num);
                break;
            }
        }

        match found {
            None => {
                println!("\nDid not find a number to give me a MD5 value starting with '00000'\n")
            }
            Some(num) => println!("The magic number is {:?}\n", num),
        }
    }
}
