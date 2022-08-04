use std::collections::{BTreeMap};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;


#[derive(Copy, Clone)]
struct Bot {
    low: Option<i32>,
    high: Option<i32>,
    low_to: Option<i32>,
    high_to: Option<i32>,
}


impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let low = match self.low {
            Some(t) => t,
            None => -1
        };
        let high = match self.high {
            Some(t) => t,
            None => -1
        };
        write!(f, "{} to {}. {} to {}.", low, self.low_to.unwrap(), high, self.high_to.unwrap())
    }
}


fn main() {
    let reader = BufReader::new(File::open("data/day10.txt").unwrap());

    let pattern = Regex::new(
        r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

    let mut bots = BTreeMap::new();

    let output_offset = 512;

    for line in reader.lines() {
        let line = line.unwrap();
        match pattern.captures(&line) {
            Some(cap) => {
                let t = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let l = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let h = cap.get(5).unwrap().as_str().parse::<i32>().unwrap();
                let mut b = bots.entry(t).or_insert(Bot { low: None, high: None, low_to: None, high_to: None });
                b.low_to = Option::from(if cap.get(2).unwrap().as_str() == "bot" { l } else { l + output_offset });
                b.high_to = Option::from(if cap.get(4).unwrap().as_str() == "bot" { h } else { h + output_offset });
            }
            None => {
                let temp = line.split(" ").collect_vec();
                let t = temp[5].parse().unwrap();
                let v = temp[1].parse().unwrap();
                let mut b = bots.entry(t).or_insert(Bot { low: None, high: None, low_to: None, high_to: None });
                if b.low.is_some() && b.low > Some(v) {
                    if b.high.is_some() { panic!() }
                    b.high = b.low;
                    b.low = Some(v);
                } else if b.low.is_some() {
                    if b.high.is_some() { panic!() }
                    b.high = Some(v);
                } else {
                    b.low = Some(v);
                }
            }
        }
    }
    let mut outputs = BTreeMap::new();
    loop {
        let mut temp = BTreeMap::new();

        for (k, v) in &bots {
            if v.low.is_some() && v.high.is_some() {
                if v.low == Some(17) && v.high == Some(61) {
                    println!("{} {}", k, v);
                }
                if v.low.unwrap() > v.high.unwrap() {
                    panic!()
                }
                let low_to = v.low_to.unwrap();
                if low_to < output_offset {
                    let mut t1 = temp
                        .entry(low_to)
                        .or_insert(bots.get(&low_to).unwrap().clone());
                    if t1.low.is_some() && t1.low.unwrap() > v.low.unwrap() {
                        if t1.high.is_some() { panic!() }
                        t1.high = t1.low;
                        t1.low = v.low;
                    } else if t1.low.is_some() {
                        if t1.high.is_some() { panic!() }
                        t1.high = v.low;
                    } else {
                        t1.low = v.low;
                    }
                } else {
                    outputs.entry(low_to - output_offset).or_insert(v.low.unwrap());
                }

                let high_to = v.high_to.unwrap();
                if high_to < output_offset {
                    let mut t2 = temp
                        .entry(high_to)
                        .or_insert(bots.get(&high_to).unwrap().clone());
                    if t2.low.is_some() && t2.low.unwrap() > v.high.unwrap() {
                        if t2.high.is_some() { panic!() }
                        t2.high = t2.low;
                        t2.low = v.high;
                    } else if t2.low.is_some() {
                        if t2.high.is_some() { panic!() }
                        t2.high = v.high;
                    } else {
                        t2.low = v.high;
                    }
                } else {
                    outputs.entry(high_to - output_offset).or_insert(v.high.unwrap());
                }
            } else {
                if !temp.contains_key(k) {
                    temp.insert(*k, *v);
                }
            }
        }

        if temp.len() == 0 { break; }
        bots = temp;
    }

    println!("{}", outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap())
}