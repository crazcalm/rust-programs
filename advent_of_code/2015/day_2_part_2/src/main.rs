use std::io;

fn main() {
    let mut ribbon = 0;

    for line in io::stdin().lines() {
        let mut dimensions: Vec<isize> = line
            .unwrap()
            .split("x")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        dimensions.sort();

        ribbon += 2 * dimensions[0]
            + 2 * dimensions[1]
            + dimensions
                .iter()
                .map(|x| x.clone())
                .reduce(|acc, e| acc * e)
                .unwrap();
    }

    println!("The elves need {} total feet of ribbon", ribbon);
}
