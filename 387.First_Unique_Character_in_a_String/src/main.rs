use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // let mut counter = HashMap::with_capacity(27);
        // s.chars().for_each(|_char| match counter.get_mut(&_char) {
        //     Some(val) => {
        //         *val += 1;
        //     }
        //     None => {
        //         counter.insert(_char, 1);
        //     }
        // });
        // // counter.

        let idx = |c: char| (c as u8 - b'a') as usize;
        let mut map = [0; 26];

        s.chars().for_each(|c| map[idx(c)] += 1);

        for (pos, c) in s.char_indices() {
            if map[idx(c)] == 1 {
                return pos as i32;
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}
