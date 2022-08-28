use itertools::Itertools;

fn extend(s: &str) -> String {
    let a = s.to_string();
    let b = s.chars().rev().map(|x| match x { '0' => '1', '1'=>'0', _ => unreachable!() }).collect::<String>();
    (a + "0" + &b).to_string()
}

fn main() {
    let mut start = "01000100010010111".to_string();
    while start.len() < 35651584 {
        start = extend(&start);
    }
    start = start.split_at(35651584).0.to_string();
    let mut checksum = start.chars().map(|x| match x { '1' => true, _ => false }).collect_vec();
    while checksum.len() % 2 == 0 {
        checksum = checksum.iter().tuple_windows().step_by(2).map(|(x, y)| x == y).collect_vec();
    }
    for i in checksum.iter() {
        let v = if *i {'1'} else {'0'};
        print!("{}", v);
    }
    println!()
}