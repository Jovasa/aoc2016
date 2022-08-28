use md5;
use hex;

fn main() {
    let secret = "ihaygndm".to_string();
    let mut end = u32::MAX;

    let mut looking_for_five: Vec<(char, u32)> = Vec::new();
    let mut valids = Vec::new();

    for i in 0.. {
        let vec = [secret.clone(), i.to_string()].concat().bytes().collect::<Vec<u8>>();
        let a =
            md5::compute(vec).to_vec();
        let mut te: String = hex::encode(a);
        for _ in 0..2016 {
            let data = te.bytes().collect::<Vec<u8>>();
            te = hex::encode(md5::compute(data).to_vec());
        }
        te
            .chars()
            .fold((0, '_'),
                  |(p_count, p_c), c|
                      if p_c == c {
                          if p_count == 2 {
                              looking_for_five.push((c, 1000));
                          }
                          else if p_count == 4 {
                              let mut to_remove = Vec::new();
                              for (indx, (o_c, u)) in looking_for_five.iter().enumerate() {
                                  if o_c == &c {
                                      if u != &1000 {
                                          println!("fouund {} {}", i, valids.len());
                                          valids.push(i + u - 1000);
                                      }
                                      to_remove.push(indx);
                                  }
                              }
                              for indx in to_remove.iter().rev() {
                                  looking_for_five.remove(*indx);
                              }
                              if valids.len() >= 64 && end == u32::MAX {
                                  end = i + 1000;
                              }
                          }
                          (p_count + 1, c)
                      } else {
                          (1, c)
                      });

        if i == end {
            break;
        }

        for t in looking_for_five.iter_mut() {
            if t.1 > 0 {
                t.1 -= 1;
            }
        }
        while looking_for_five.len() != 0 && looking_for_five[0].1 == 0 {
            looking_for_five.remove(0);
        }
    }

    valids.sort();
    for v in valids {
        println!("{v}");
    }
    // println!("{} {}", valids[63], valids.len());
}