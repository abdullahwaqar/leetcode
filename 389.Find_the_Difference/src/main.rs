struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ret = 0;

        for _char in s.as_bytes() {
            ret ^= _char;
        }

        for _char in t.as_bytes() {
            ret ^= _char;
        }

        return ret as char;
    }
}

fn main() {
    println!("Hello, world!");
}
