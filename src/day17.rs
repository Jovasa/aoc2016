use std::collections::HashSet;

fn main() {
    let secret = "lpvhkcbi".to_string();

    let mut positions = HashSet::new();
    positions.insert((0, 0, secret.to_string()));

    let mut longest = 0;

    while positions.len() > 0 {
        let mut temp = HashSet::new();
        for (x, y, s) in positions {
            if x == 3 && y == 3 {
                let temp_length = s.len() - secret.len();
                if longest == 0 {
                    println!("{}", s);
                }
                if temp_length > longest {
                    longest = temp_length;
                }
                continue;
            }
            let vec = s.bytes().collect::<Vec<u8>>();
            let a = md5::compute(vec).to_vec();
            let te: String = hex::encode(a);
            if x != 3 && te.chars().nth(3).unwrap().to_digit(16).unwrap() > 10 {
                temp.insert((x + 1, y, [&s, "R"].concat().to_string()));
            }
            if x != 0 && te.chars().nth(2).unwrap().to_digit(16).unwrap() > 10 {
                temp.insert((x - 1, y, [&s, "L"].concat().to_string()));
            }
            if y != 3 && te.chars().nth(1).unwrap().to_digit(16).unwrap() > 10 {
                temp.insert((x, y + 1, [&s, "D"].concat().to_string()));
            }
            if y != 0 && te.chars().nth(0).unwrap().to_digit(16).unwrap() > 10 {
                temp.insert((x, y - 1, [&s, "U"].concat().to_string()));
            }
        }
        positions = temp;
    }
    println!("{}", longest);
}