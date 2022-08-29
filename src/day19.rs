use std::collections::VecDeque;

fn main() {
    first();
    second();
}

fn first() {
    let end = 3005290;
    let mut distance_until_next_present = (0..end).map(|_| 1).collect::<Vec<usize>>();

    let mut current_pos = 0;
    while distance_until_next_present[current_pos] != end {
        let current_distance = distance_until_next_present[current_pos];
        let offset = (current_distance + current_pos) % end;
        distance_until_next_present[current_pos] = distance_until_next_present[offset] + current_distance;
        current_pos = (current_pos + distance_until_next_present[current_pos]) % end;
    }

    println!("{}", current_pos + 1);
}

fn second() {
    let end = 3005290;
    let mut order = (1..=end).collect::<VecDeque<usize>>();

    let mut index = 0;
    while order.len() > 1 {
        let to_remove = (index + order.len() / 2) % order.len();
        order.remove(to_remove);
        if to_remove >= index {
            index += 1;
        }
        index %= order.len();
    }
    println!("{}", order[0]);
}