use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    first();
    second();
}

fn first() {
    let reader = BufReader::new(File::open("data/day7.txt").unwrap());

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let t = line
            .split(|x| x == '[' || x == ']')
            .map(|a|
                a.chars().tuple_windows()
                    .map(|(x, y, z, w)| x == w && y == z && x != y)
                    .fold(false, |p, n| p || n)
            )
            .enumerate()
            .fold((false, false), |p, (i, n)| {
                let x3 = i % 2 == 0 && n;
                let x4 = i % 2 == 1 && n;
                (x3 || p.0, x4 || p.1)
            });
        if t.0 && !t.1 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn second() {
    let reader = BufReader::new(File::open("data/day7.txt").unwrap());

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut even = Vec::new();
        let mut odd = Vec::new();
        line
            .split(|x| x == '[' || x == ']')
            .enumerate()
            .for_each(|(i, a)|
                a
                    .chars()
                    .tuple_windows()
                    .filter(|(x, y, z)| x == z && x != y)
                    .for_each(|(x, y, z)| {
                        if i % 2 == 0 {
                            even.push((x, y));
                        } else {
                            odd.push((y, x));
                        }
                    })
            );
        let mut found = false;
        for t in even {
            found |= odd.contains(&t);
        }
        if found { count += 1 };
    }
    println!("{}", count);
}