use std::collections::HashMap;
use md5;
use hex;

fn main() {
    let secret = "ihaygndm".to_string();
    let mut end = u32::MAX;

    let mut looking_for_five: Vec<(char, u32)> = Vec::new();
    let mut valids =

    for i in 0.. {
        let a =
            md5::compute([secret.clone(), i.to_string()].concat().bytes().collect::<Vec<u8>>()).to_vec();
        let hex: String = hex::encode(a);

        hex
            .chars()
            .fold((0, '_'),
                  |(p_count, p_c), c|
                      if p_c == c {
                          if p_count == 2 {
                              looking_for_five.push((c, 1000));
                          }
                          else if p_count == 4 {
                              for (o_c, u) in looking_for_five {
                                  if o_c == c {

                                  }
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
        while looking_for_five[0].1 == 0 {
            looking_for_five.remove(0);
        }
    }
}