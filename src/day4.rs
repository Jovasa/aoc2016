use std::collections::{BTreeMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day4.txt").unwrap());

    let mut total = 0;

    let range = ('z' as u8 - 'a' as u8 + 1) as i32;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut counts = BTreeMap::new();
        let mut tail = "".to_owned();
        let mut parts = Vec::new();
        for l in line.split("-") {
            if l.contains("[") {
                tail = l.to_owned();
                break;
            }

            l.chars().for_each(
                |x| *counts.entry(x).or_insert(0) += 1
            );
            parts.push(l);
        }
        let mut temp = tail.split("[");
        let id = temp.next().unwrap().parse::<i32>().unwrap();
        let checksum = temp.next().unwrap().strip_suffix("]").unwrap();

        let mut is_valid = true;
        for ((_k, v), a) in counts
            .iter()
            .map(|(&k, &v)| (v * 27 + (27 - (k as i32 - 'a' as i32)), k))
            .sorted()
            .rev()
            .take(5)
            .zip(checksum.chars()){
            is_valid &= v == a;
        }
        if is_valid {
            total += id;
            let cipher = parts
                .concat()
                .chars()
                .map(|x| {
                    let t = (id % range) as u8;
                    let t1 = x as u8 + t;
                    if t1 > 'z' as u8 {
                        (t1 - range as u8) as char
                    }
                    else {
                        t1 as char
                    }
                })
                .collect::<String>();
            if cipher == "northpoleobjectstorage".to_owned(){
                println!("{}", id);
            }
        }


    }
    println!("{}", total);
}