use std::io;

fn main() {
    let mut wrapping_paper = 0;

    for line in io::stdin().lines() {
        let dimensions: Vec<isize> = line
            .unwrap()
            .split("x")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);

        let side_1 = 2 * l * w;
        let side_2 = 2 * w * h;
        let side_3 = 2 * h * l;
        let extra = {
            let mut temp = vec![l * w, w * h, h * l];
            temp.sort();
            temp[0]
        };

        wrapping_paper += side_1 + side_2 + side_3 + extra;
    }
    println!(
        "The elves need {} square feet of wrapping paper",
        wrapping_paper
    );
}
