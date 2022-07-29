use md5;

fn main() {
    let secret = "abbhdwsy".to_string();
    let mut count = 0;
    let mut things = [255; 8];
    for i in 1.. {
        let temp = md5::compute([secret.clone(), i.to_string()].concat().bytes().collect::<Vec<u8>>()).to_vec();
        if temp[0] == 0 && temp[1] == 0 && temp[2] & 0xf0 == 0 {
            println!("five {} {}", temp[2] & 0x0f, (temp[3] & 0xf0) >> 4);
            count += 1;
            let temp2 = ((temp[3] & 0xf0) >> 4);
            let val = (temp[2] & 0x0f) as usize;

            if val < 8 && things[val] == 255{
                things[val] = temp2;
            }

            if ! things.iter().fold(false, |p, &n| p || n == 255) { break }
        }
    }

    for a in things.iter() {
        println!("{}", a);
    }
}