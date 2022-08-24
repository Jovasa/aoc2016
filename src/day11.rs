use std::collections::HashSet;

const GENERATOR_OFFSET: u64 = 5;
const CHIPS: u64 = (1 << GENERATOR_OFFSET) - 1;
const FLOOR_OFFSET: u64 = GENERATOR_OFFSET * 2;
const FLOOR: u64 = (1 << FLOOR_OFFSET) - 1;

const POLONIUM: u64 = 0;
const THULIUM: u64 = 1;
const PROMETHIUM: u64 = 2;
const RUTHENIUM: u64 = 3;
const COBALT: u64 = 4;

const FLOOR1: u64 = FLOOR_OFFSET * 0;
const FLOOR2: u64 = FLOOR_OFFSET * 1;
const FLOOR3: u64 = FLOOR_OFFSET * 2;
const FLOOR4: u64 = FLOOR_OFFSET * 3;

const CURRENT_FLOOR_OFFSET: u64 = 60;

fn is_valid_floor(in_: u64) -> bool {
    let both = in_ & (in_ >> GENERATOR_OFFSET);
    if both == 0 {
        return true;
    }
    (both ^ (in_ & CHIPS)) == 0
}

fn check_all_floors(in_: u64) -> bool {
    (0..4)
        .map(|x| is_valid_floor((in_ >> (FLOOR_OFFSET * x)) & FLOOR))
        .fold(true, |a, p| a && p)
}

fn main() {
    let start: u64 = 1 << (POLONIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (POLONIUM + FLOOR2) |
        1 << (THULIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (THULIUM + FLOOR1) |
        1 << (PROMETHIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (PROMETHIUM + FLOOR2) |
        1 << (RUTHENIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (RUTHENIUM + FLOOR1) |
        1 << (COBALT + GENERATOR_OFFSET + FLOOR1) |
        1 << (COBALT + FLOOR1) |
        1 << CURRENT_FLOOR_OFFSET;

    let mut iters = 0;
    let mut current_set = HashSet::new();

    current_set.insert(start);

    loop {
        let mut temp = HashSet::new();
        for item in current_set {
            let current_floor = item >> CURRENT_FLOOR_OFFSET;
            temp.insert(item);
            if item == 4 << CURRENT_FLOOR_OFFSET | (FLOOR << FLOOR3) {
                println!("{}", iters);
                return;
            }
        }
        current_set = temp;
        iters += 1;
    }
}