use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;


fn main() {
    let reader = BufReader::new(File::open("data/day24.txt").unwrap());

    let floor_plan = reader.lines().map(
        |x| x.unwrap().chars().collect_vec()
    ).collect_vec();

    let mut places = Vec::new();

    for (y, row) in floor_plan.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                places.push(((y, x), c.to_digit(10).unwrap()));
            }
        }
    }

    let mut distances = HashMap::new();

    for (p, d) in places {
        let mut visited = HashSet::new();
        visited.insert(p);

        let mut distance_from_start = 1;
        let mut possible_to_visit = HashSet::new();
        possible_to_visit.insert(p);
        while possible_to_visit.len() != 0 {
            let mut temp = HashSet::new();
            for (y, x) in possible_to_visit {
                if x != 0 {
                    check(&floor_plan, &mut distances, d, distance_from_start, &mut temp, &mut visited, x - 1, y);
                }
                if y != 0 {
                    check(&floor_plan, &mut distances, d, distance_from_start, &mut temp, &mut visited, x, y - 1);
                }
                if x != floor_plan[0].len() - 1 {
                    check(&floor_plan, &mut distances, d, distance_from_start, &mut temp, &mut visited, x + 1, y);
                }
                if y != floor_plan.len() - 1 {
                    check(&floor_plan, &mut distances, d, distance_from_start, &mut temp, &mut visited, x, y + 1);
                }
            }
            possible_to_visit = temp;
            distance_from_start += 1;
        }
    }

    for (k, v) in distances {
        println!("{} -> {} : {}", k.0, k.1, v);
    }
}

fn check(floor_plan: &Vec<Vec<char>>,
         distances: &mut HashMap<(u32, u32), i32>,
         d: u32,
         distance_from_start: i32,
         temp: &mut HashSet<(usize, usize)>,
         visited: &mut HashSet<(usize, usize)>,
         x: usize, y: usize) {
    if visited.contains(&(y, x)) {
        return;
    }
    match floor_plan[y][x] {
        '.' => {
            temp.insert((y, x));
            visited.insert((y, x));
        }
        '#' => {}
        a => {
            let l = a.to_digit(10).unwrap();
            distances.insert((d, l), distance_from_start);
            distances.insert((l, d), distance_from_start);
            temp.insert((y, x));
            visited.insert((y, x));
        }
    };
}