use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    first();
    second();
}

fn first() {
    let reader = BufReader::new(File::open("data/day3.txt").unwrap());

    let valids = reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            x
                .split_whitespace()
                .map(|x|
                    x.parse::<i32>().unwrap()
                )
                .sorted()
                .collect::<Vec<i32>>()
        })
        .filter(|a| a[0] + a[1] > a[2])
        .collect_vec();

    println!("{}", valids.len());
}

fn second() {
    let reader = BufReader::new(File::open("data/day3.txt").unwrap());

    let mut i = reader.lines();

    let mut valids = 0;
    loop {
        let a = match i.next() {
            Some(t) => t.unwrap(),
            None => break,
        }.split_whitespace().map(|q| q.parse::<i32>().unwrap()).collect_vec();
        let b = i.next().unwrap().unwrap()
            .split_whitespace().map(|q| q.parse::<i32>().unwrap()).collect_vec();
        let c = i.next().unwrap().unwrap()
            .split_whitespace().map(|q| q.parse::<i32>().unwrap()).collect_vec();

        for x in 0..3 {
            let mut temp = vec![a[x], b[x], c[x]];
            temp.sort();
            valids += (temp[0] + temp[1] > temp[2]) as i32;
        }
    }

    println!("{}", valids);
}