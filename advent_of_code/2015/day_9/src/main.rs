use std::collections::{HashMap, HashSet};
use std::io;

fn update_distances(
    mut distances: HashMap<String, HashMap<String, usize>>,
    start: String,
    end: String,
    distance: usize,
) -> HashMap<String, HashMap<String, usize>> {
    match distances.contains_key(&start) {
        false => {
            let mut temp = HashMap::new();
            temp.insert(end.clone(), distance.clone());
            distances.insert(start.clone(), temp);
        }
        true => {
            if let Some(data) = distances.get_mut(&start) {
                data.insert(end.clone(), distance.clone());
            }
        }
    }

    distances
}

fn create_data_structure(
    mut distances: HashMap<String, HashMap<String, usize>>,
) -> HashMap<String, HashMap<String, usize>> {
    for line in io::stdin().lines() {
        let data = line.unwrap();
        let data: Vec<String> = data.split(" ").map(|x| x.to_string()).collect();

        let (start, end, distance) = (data[0].clone(), data[2].clone(), data[4].clone());
        let distance: usize = distance.parse().unwrap();

        // updating the map
        distances = update_distances(distances, start.clone(), end.clone(), distance.clone());
        distances = update_distances(distances, end.clone(), start.clone(), distance.clone());
        println!("{start} {end} {distance}");
    }

    distances
}

fn closest_distance(
    map: &HashMap<String, usize>,
    seen: &HashSet<String>,
) -> Option<(String, usize)> {
    let mut result = None;
    let mut min = 1000;
    for (loc, dis) in map.iter() {
        if !seen.contains(loc) {
            if dis < &min {
                min = dis.clone();
                result = Some((loc.to_string(), dis.clone()));
            }
        }
    }
    result
}

fn main() {
    let mut distances: HashMap<String, HashMap<String, usize>> = HashMap::new();
    distances = create_data_structure(distances);

    let mut shortest_distance = 10000;
    for (start, map) in distances.iter() {
        print!("Start {start} -> ");

        let mut current_loc = start.clone();
        let mut current_map = map;
        let mut total = 0;
        let mut seen = HashSet::new();
        while seen.len() < map.len() {
            seen.insert(current_loc.clone());
            match closest_distance(current_map, &seen) {
                None => {}
                Some((loc, dis)) => {
                    total += dis;
                    print!("{loc} ({total}) -> ");

                    current_loc = loc;
                    current_map = distances.get(&current_loc).unwrap()
                }
            }
        }
        if total < shortest_distance {
            shortest_distance = total;
        }
        println!("\n");
    }

    println!("{}", shortest_distance);
}
