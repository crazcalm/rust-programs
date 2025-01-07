use std::io;

struct Light {
    brightness: usize,
}

impl Light {
    fn new() -> Self {
        Self { brightness: 0 }
    }

    fn brightness(&self) -> usize {
        self.brightness
    }

    fn toggle(&mut self) {
        self.brightness += 2;
    }

    fn turn_on(&mut self) {
        self.brightness += 1;
    }

    fn turn_off(&mut self) {
        match self.brightness {
            0 => {}
            _ => self.brightness -= 1,
        };
    }
}

fn parse(data: &Vec<&str>) -> (String, (usize, usize), (usize, usize)) {
    let len = data.len();

    let mut command = data[0].to_string();
    if len == 5 {
        command = format!("{} {}", data[0], data[1]);
    }

    let start: Vec<usize> = data[len - 3]
        .to_string()
        .as_str()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let end: Vec<usize> = data[len - 1]
        .to_string()
        .as_str()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    return (command, (start[0], start[1]), (end[0], end[1]));
}

fn create_grid() -> Vec<Vec<Light>> {
    let mut vec_1 = Vec::with_capacity(999);
    for _ in 0..=999 {
        let mut vec_2 = Vec::with_capacity(999);

        for _ in 0..=999 {
            vec_2.push(Light::new());
        }
        vec_1.push(vec_2);
    }

    return vec_1;
}

fn change_lights(
    command: String,
    data: &mut Vec<Vec<Light>>,
    coord_1: (usize, usize),
    coord_2: (usize, usize),
) {
    for x in coord_1.0..=coord_2.0 {
        for y in coord_1.1..=coord_2.1 {
            if command.eq(&"turn on") {
                data[x][y].turn_on();
            } else if command.eq(&"turn off") {
                data[x][y].turn_off();
            } else if command.eq(&"toggle") {
                data[x][y].toggle();
            }
        }
    }
}

fn main() {
    let mut grid = create_grid();
    for line in io::stdin().lines() {
        let data = line.unwrap();
        let data: Vec<&str> = data.split(" ").collect();

        println!("data: {:?} ", data);
        let (command, coord_1, coord_2) = parse(&data);
        println!("{} {:?} {:?}", command, coord_1, coord_2);

        change_lights(command, &mut grid, coord_1, coord_2);
    }

    // How many lights are on
    let mut count = 0;
    for row in grid.iter() {
        for light in row.iter() {
            count += light.brightness;
        }
    }

    println!("The total brightness of the lights is {}", count);
}
