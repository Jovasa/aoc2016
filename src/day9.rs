use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day9.txt").unwrap();
    println!("{}", decompress(contents).len())
}

fn decompress(contents: String) -> String {
    let mut out = Vec::new();


    let mut iter = contents.chars();
    loop {
        let c = match iter.next() {
            Some(c) => c,
            None => break,
        };
        if c != '(' {
            out.push(c);
        } else {
            let mut times = 0;
            let mut chars = 0;
            let mut temp = iter.next().unwrap();
            while temp != 'x' {
                chars *= 10;
                chars += temp.to_digit(10).unwrap();
                temp = iter.next().unwrap();
            }
            let mut temp = iter.next().unwrap();
            while temp != ')' {
                times *= 10;
                times += temp.to_digit(10).unwrap();
                temp = iter.next().unwrap();
            }
            let buffer = {
                let mut b = Vec::new();
                for _ in 0..chars {
                    b.push(iter.next().unwrap());
                }
                b
            };
            for _ in 0..times {
                out.extend(buffer.iter());
            }
        }
    }
    out.iter().collect::<String>()
}