fn extend(s: &str) -> String {
    let a = s.to_string();
    let b = s.chars().rev().map(|x| match x { '0' => '1', '1'=>'0', _ => unreachable!() }).collect::<String>();
    (a + "0" + &b).to_string()
}

fn main() {
    let mut start = "01000100010010111".to_string();
    while start.len() < 272 {
        start = extend(&start);
    }
    start = start.split_at(272).0.to_string();
    let mut checksum = start;
    while checksum.len() % 2 == 0 {
        let mut temp = Vec::new();
        for i in 0..checksum.len()/2 {
            let t = checksum.chars().nth(i*2).unwrap();
            let t2 = checksum.chars().nth(i*2 + 1).unwrap();
            if t == t2 {temp.push('1')} else { temp.push('0') }
        }
        checksum = temp.iter().collect();
    }
    println!("{}", checksum);
}