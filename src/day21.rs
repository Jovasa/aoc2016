use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let mut pw = "abcdefgh".chars().collect_vec();
    //let mut pw = "fbgdceah".chars().collect_vec();

    let reader = BufReader::new(File::open("data/day21.txt").unwrap());
    let lines = reader.lines().map(|x| x.unwrap()).collect_vec();
    for line in lines.iter() {
        let parts = line.split(" ").collect_vec();
        match parts[0] {
            "reverse" => {
                let mut start = parts[2].parse::<usize>().unwrap();
                let mut end = parts[4].parse::<usize>().unwrap();
                while start < end {
                    let temp = pw[start];
                    pw[start] = pw[end];
                    pw[end] = temp;
                    start += 1; end -= 1;
                }
            },
            "rotate" => {
                match parts[1] {
                    "left" => {
                        let amount = parts[2].parse::<usize>().unwrap();
                        pw.rotate_left(amount);
                    },
                    "right" => {
                        let amount = parts[2].parse::<usize>().unwrap();
                        pw.rotate_right(amount);
                    },
                    "based" => {
                        let mut amount = pw.
                            iter()
                            .position(|x| x == &parts[6]
                                .chars()
                                .nth(0)
                                .unwrap())
                            .unwrap() + 1;
                        if amount > 4 {amount += 1}

                        let len = pw.len();
                        pw.rotate_right(amount % len);
                    },
                    _ => unreachable!()
                }
            },
            "move" => {
                let start = parts[2].parse::<usize>().unwrap();
                let end = parts[5].parse::<usize>().unwrap();
                let t = pw.remove(start);
                pw.insert(end, t);
            },
            "swap" => {
                let first = if parts[2].chars().nth(0).unwrap().is_digit(10) {
                    parts[2].parse::<usize>().unwrap()
                } else {
                    pw.iter().position(|x| x == &parts[2].chars().nth(0).unwrap()).unwrap()
                };
                let second = if parts[5].chars().nth(0).unwrap().is_digit(10) {
                    parts[5].parse::<usize>().unwrap()
                } else {
                    pw.iter().position(|x| x == &parts[5].chars().nth(0).unwrap()).unwrap()
                };
                let temp = pw[first];
                pw[first] = pw[second];
                pw[second] = temp;
            }
            _ => unreachable!()
        }
    }
    println!("{}", pw.iter().collect::<String>());
}