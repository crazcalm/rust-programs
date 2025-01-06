use std::io;

use md5;

fn main() {
    for line in io::stdin().lines() {
        let mut found = None;

        let key = line.as_ref().unwrap();
        print!("{key}");

        for num in 0..=2000000 {
            let digest = md5::compute(format!("{}{}", line.as_ref().unwrap(), num));

            if format!("{:x}", digest).starts_with("00000") {
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
