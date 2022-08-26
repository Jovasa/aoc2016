use std::collections::HashSet;

fn is_empty(x: u32, y: u32) -> bool {
    let temp = x*x + 3*x + 2*x*y + y + y*y + 1358;
    temp.count_ones() % 2 == 0
}

fn can_add(x: u32, y: u32, visited: &HashSet<(u32, u32)>) -> bool {
    !visited.contains(&(x, y)) && is_empty(x, y)
}


fn main() {

    let mut visited = HashSet::new();

    let mut current_points = Vec::new();
    current_points.push((1, 1));
    visited.insert((1, 1));

    let mut iters = 0;
    loop {
        let mut temp = Vec::new();
        for (x, y) in current_points {
            if x == 31 && y == 39 {
                println!("{iters}");
                return;
            }
            if can_add(x + 1, y, &visited) {
                temp.push((x + 1, y));
                visited.insert((x + 1, y));
            }
            if can_add(x, y + 1, &visited) {
                temp.push((x, y + 1));
                visited.insert((x, y  + 1));
            }
            if x != 0 && can_add(x - 1, y, &visited) {
                temp.push((x - 1, y));
                visited.insert((x - 1, y));
            }
            if y != 0 && can_add(x, y - 1, &visited) {
                temp.push((x, y - 1));
                visited.insert((x, y - 1));
            }
        }
        if iters == 49 {
            println!("{}", visited.len());
        }
        current_points = temp;
        iters += 1;
    }
}