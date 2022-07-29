use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day6.txt").unwrap());

    let mut counters = vec![BTreeMap::new(); 8];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut i = line.chars();
        for x in 0..line.len() {
            let r = i.next().unwrap();
            *counters[x].entry(r).or_insert(0) += 1;
        }
    }

    for x in 0..8 {
        print!("{}", counters[x].iter().map(|(k, v)| (v, k)).sorted().rev().next().unwrap().1);
    }
    println!();
    for x in 0..8 {
        print!("{}", counters[x].iter().map(|(k, v)| (v, k)).sorted().next().unwrap().1);
    }
    println!();
}