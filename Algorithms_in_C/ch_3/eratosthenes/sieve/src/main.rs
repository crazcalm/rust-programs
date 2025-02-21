const N: usize = 1000;

fn main() {
    let (mut i, mut j) = (2, 3);
    let mut a: [bool; N + 1] = [false; N + 1];

    // Setting up the initial array
    a[1] = false;
    for value in a.iter_mut() {
        *value = true
    }

    // We know that i*j is not prime,
    // So for each product we can create,
    // we set that index's value to false
    while i <= N / 2 {
        while j <= N / i {
            a[i * j] = false;
            j += 1;
        }
        j = 2;
        i += 1;
    }

    // Printing out all the primes
    for index in 2..N + 1 {
        if a[index] {
            print!("{:4}", index);
        }
    }
    print!("\n");
}
