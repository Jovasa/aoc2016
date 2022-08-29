use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day20.txt").unwrap());

    let mut ranges = reader
        .lines()
        .map(
            |x| {
                let x = x.unwrap();
                let mut s = x.split("-");
                (s.next().unwrap().parse::<usize>().unwrap(), s.next().unwrap().parse::<usize>().unwrap())
            }
        ).collect_vec();
    ranges.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    let mut end = 0;
    let mut not_blacklisted = 0;
    for (low, high) in ranges {
        if end + 1 >= low {
            end = high.max(end);
        }
        else {
            println!("{}", end+ 1);
            not_blacklisted += low - end - 1;
            end = high
        }
    }
    println!("{}", not_blacklisted);
}