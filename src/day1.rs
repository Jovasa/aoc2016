use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() {
    let data = fs::read_to_string("data/day1.txt").unwrap();

    let mut direction = 0;
    let mut location: (i32, i32) = (0,0);

    let mut locations_visited = HashSet::new();

    for d in data.strip_suffix("\n").unwrap().split(", ").collect_vec() {
        let da = d.chars().next().unwrap();
        let v: i32 = d[1..].parse().unwrap();
        match da {
            'R' => direction += 1,
            'L' => direction -= 1,
            _ => unreachable!()
        };
        direction = (direction + 4) % 4;

        for _ in 1..=v {
            match direction {
                0 => location.1 += 1,
                1 => location.0 += 1,
                2 => location.1 -= 1,
                3 => location.0 -= 1,
                _ => unreachable!()
            }
            if locations_visited.contains(&location) {
                println!("{}", location.0.abs() + location.1.abs());
            }
            locations_visited.insert(location);
        }
    }
    println!("{}", location.0.abs() + location.1.abs());
}