use std::io;

fn main() {
    println!("Set N and M (example: '9 5'):");

    // Capture user input
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    // Parse user input
    let data: Vec<usize> = buffer
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse().expect("Expect a digit"))
        .collect();

    #[allow(non_snake_case)]
    let N = data[0];
    #[allow(non_snake_case)]
    let M = data[1];

    // Create List
    let mut list: Vec<usize> = (1..=N).collect();

    // Josephus problem
    while list.len() > 1 {
        for _ in 1..M {
            // Rotate the items in the list to the left
            list.rotate_left(1);
        }
        let data = list.remove(0);
        print!("{data} ")
    }
    print!("{}", list[0]);
}
