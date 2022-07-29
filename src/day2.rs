use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    first();
    second();
}

fn first() {
    let reader = BufReader::new(File::open("data/day2.txt").unwrap());

    let mut val = 5;

    for line in reader.lines() {
        let line = line.unwrap();
        val = line.chars().fold(val, |val, n| {
            match n {
                'D' => if val <= 6 { val + 3 } else { val },
                'U' => if val >= 4 { val - 3 } else { val },
                'L' => if val % 3 != 1 { val - 1 } else { val },
                'R' => if val % 3 != 0 { val + 1 } else { val },
                _ => unreachable!()
            }
        });
        println!("{}", val);
    }
}

fn second() {
    let reader = BufReader::new(File::open("data/day2.txt").unwrap());
    let vals = vec![
        0, 0, 1, 0, 0,
        0, 2, 3, 4, 0,
        5, 6, 7, 8, 9,
        0, 10, 11, 12, 0,
        0, 0, 13, 0, 0
    ];
    let mut pos: (i32, i32)  = ( -2, 0);
    println!("Second");
    for line in reader.lines() {
        let line = line.unwrap();
        pos = line.chars().fold(pos, |pos, n| {
            match n {
                'U' => if (pos.1 - 1).abs() + pos.0.abs() <= 2 { (pos.0, pos.1 - 1) } else { pos },
                'D' => if (pos.1 + 1).abs() + pos.0.abs() <= 2 { (pos.0, pos.1 + 1) } else { pos },
                'L' => if (pos.0 - 1).abs() + pos.1.abs() <= 2 { (pos.0 - 1, pos.1) } else { pos },
                'R' => if (pos.0 + 1).abs() + pos.1.abs() <= 2 { (pos.0 + 1, pos.1) } else { pos },
                _ => unreachable!()
            }
        });
        println!("{}", vals[(pos.0 + 2 + (pos.1 + 2) * 5) as usize]);
    }
}