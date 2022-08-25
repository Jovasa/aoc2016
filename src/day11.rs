use std::collections::HashSet;

const GENERATOR_OFFSET: u128 = 7;
const CHIPS: u128 = (1 << GENERATOR_OFFSET) - 1;
const FLOOR_OFFSET: u128 = GENERATOR_OFFSET * 2;
const FLOOR: u128 = (1 << FLOOR_OFFSET) - 1;

const POLONIUM: u128 = 0;
const THULIUM: u128 = 1;
const PROMETHIUM: u128 = 2;
const RUTHENIUM: u128 = 3;
const COBALT: u128 = 4;
const ELERIUM: u128 = 5;
const DILITHIUM: u128 = 6;

const FLOOR1: u128 = FLOOR_OFFSET * 0;
const FLOOR2: u128 = FLOOR_OFFSET * 1;
const FLOOR3: u128 = FLOOR_OFFSET * 2;
const FLOOR4: u128 = FLOOR_OFFSET * 3;

const CURRENT_FLOOR_OFFSET: u128 = 100;


fn is_valid_floor(in_: u128) -> bool {
    let both = in_ & (in_ >> GENERATOR_OFFSET);
    if both == 0 {
        return true;
    }
    (both ^ (in_ & CHIPS)) == 0
}

fn check_all_floors(in_: u128) -> bool {
    (0..4)
        .map(|x| is_valid_floor((in_ >> (FLOOR_OFFSET * x)) & FLOOR))
        .fold(true, |a, p| a && p)
}

fn main() {
    let start: u128 = 1 << (POLONIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (POLONIUM + FLOOR2) |
        1 << (THULIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (THULIUM + FLOOR1) |
        1 << (PROMETHIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (PROMETHIUM + FLOOR2) |
        1 << (RUTHENIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (RUTHENIUM + FLOOR1) |
        1 << (COBALT + GENERATOR_OFFSET + FLOOR1) |
        1 << (COBALT + FLOOR1) |
        1 << (ELERIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (ELERIUM + FLOOR1) |
        1 << (DILITHIUM + GENERATOR_OFFSET + FLOOR1) |
        1 << (DILITHIUM + FLOOR1) |
        0 << CURRENT_FLOOR_OFFSET;

    let mut iters = 0;
    let mut current_set = HashSet::new();

    current_set.insert(start);

    let mut all = HashSet::new();

    loop {
        let mut temp = HashSet::new();
        for item in current_set {
            let current_floor = item >> CURRENT_FLOOR_OFFSET;
            for chip in 0..GENERATOR_OFFSET {
                let c = item & (1 << (chip + current_floor * FLOOR_OFFSET));
                if c != 0 {
                    let current_chip_removed = item ^ c ^ (current_floor << CURRENT_FLOOR_OFFSET);

                    for obj in 0..FLOOR_OFFSET {
                        let o = current_chip_removed & (1 << (obj + current_floor * FLOOR_OFFSET));
                        if o != 0 || chip == obj {
                            let sec_obj_removed = current_chip_removed ^ o;
                            if current_floor != 0 {
                                let new_floor = current_floor - 1;
                                let new_state = sec_obj_removed |
                                    new_floor << CURRENT_FLOOR_OFFSET |
                                    c >> FLOOR_OFFSET |
                                    o >> FLOOR_OFFSET;
                                if check_all_floors(new_state) && !all.contains(&new_state) {
                                    temp.insert(new_state);
                                    all.insert(new_state);
                                }
                            }
                            if current_floor != 3 {
                                let new_floor = current_floor + 1;
                                let new_state = sec_obj_removed |
                                    new_floor << CURRENT_FLOOR_OFFSET |
                                    c << FLOOR_OFFSET |
                                    o << FLOOR_OFFSET;
                                if check_all_floors(new_state) && !all.contains(&new_state) {
                                    temp.insert(new_state);
                                    all.insert(new_state);
                                }
                            }
                        }
                    }
                }
            }
            if item == 3 << CURRENT_FLOOR_OFFSET | (FLOOR << FLOOR4) {
                println!("{}", iters);
                return;
            }
        }
        println!("{} {}", iters, temp.len());
        if temp.len() == 0 {
            println!("Failed after {iters} iterations");
            break;
        }
        current_set = temp;
        iters += 1;
    }
}