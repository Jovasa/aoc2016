use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let pattern = Regex::new(r"Disc #\d has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    let reader = BufReader::new(File::open("data/day15.txt").unwrap());

    let dics = reader
        .lines()
        .enumerate()
        .map(|(n, p)| {
            let p = p.unwrap();
            let cap = pattern.captures(&p).unwrap();
            (cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap(), n + 1)
        }).collect_vec();

    let d = dics.iter().fold((1, 0), |(cycle, offset), (positions, start, num)| {
        let mut hole_pos = offset;
        while (hole_pos + num + *start) % positions != 0 {
            hole_pos += cycle;
        }
        (cycle * positions, hole_pos)
    });
    println!("{}", d.1);
}