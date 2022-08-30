use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let pattern = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();

    let reader = BufReader::new(File::open("data/day22.txt").unwrap());

    let mut nodes = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let cap = pattern.captures(&line);
        match cap {
            Some(x) => {
                let vec = x.iter().skip(1).map(|x| x.unwrap().as_str().parse::<i32>().unwrap()).collect_vec();
                nodes.push(vec)
            },
            None => {}
        }
    }

    let mut count = 0;

    for (i, node) in nodes.iter().enumerate() {
        for j in i+1..nodes.len() {
            let temp = &nodes[j];
            if temp[3] != 0 && temp[3] < node[4] {
                count += 1;
            }
            if node[3] != 0 && temp[4] > node[3] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}