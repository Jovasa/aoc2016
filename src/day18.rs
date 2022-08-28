use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day18.txt").unwrap());

    let mut current_row = Vec::new();
    current_row.push(true);
    for line in reader.lines() {
        let line = line.unwrap();
        line.chars().for_each(|x| if x == '^' {current_row.push(false)} else {current_row.push(true)})
    }
    current_row.push(true);

    let mut safe_tiles = 0;
    for _ in 0..400000 {
        let mut temp = Vec::new();
        temp.push(true);
        current_row.iter().for_each(|x| safe_tiles += *x as u32);
        safe_tiles -= 2;
        for i in 1..(current_row.len() - 1) {
            let l = current_row[i - 1];
            let c = current_row[i];
            let r = current_row[i + 1];

            if (l && !c && !r) || (!l && !c && r) || (l && c && !r) || (!l && c && r) {
                temp.push(false);
            } else {
                temp.push(true);
            }
        }
        temp.push(true);
        current_row = temp;
    }
    println!("{}", safe_tiles);
}