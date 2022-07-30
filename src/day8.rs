use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut grid = vec![false; 50*6];
    let reader = BufReader::new(File::open("data/day8.txt").unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        match line.split_once(" ") {
            Some((f, s)) => {
                if f == "rect" {
                    let (x_limit, y_limit) = match s.split_once("x") {
                        Some((x, y)) => (x.parse().unwrap(), y.parse().unwrap()),
                        None => unreachable!()
                    };
                    for y in 0..y_limit {
                        for x in 0..x_limit {
                            grid[x + y * 50] = true;
                        }
                    }
                }
                else {
                    match s.split_once("=") {
                        Some((t, r)) => {
                            let (place, amount) : (usize, usize) = match r.split_once(" by ") {
                                Some((a, b)) => (a.parse().unwrap(), b.parse().unwrap()),
                                None => unreachable!()
                            };
                            let mut temp = [false; 50];
                            if t == "row y" {
                                for x in 0..50 {
                                    temp[(x + amount) % 50] = grid[place * 50 + x];
                                }
                                for x in 0..50 {
                                    grid[place * 50 + x] = temp[x];
                                }
                            }
                            else {
                                for y in 0..6 {
                                    temp[(y + amount) % 6] = grid[place + 50 * y];
                                }
                                for y in 0..6 {
                                    grid[place + 50 * y] = temp[y];
                                }
                            }
                        }
                        None => unreachable!()
                    }
                }
            }
            None => unreachable!()
        };
    }

    let count = grid.iter().fold(0, |p, &n | p + n as i32);
    println!("{}", count);
    for y in 0..6 {
        for x in 0..50 {
            print!("{}", if grid[y * 50 + x] { "#" } else { "." })
        }
        println!()
    }
    println!()
}