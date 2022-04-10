struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut n = 0;
        let mut c_swap = (' ', ' ');

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                n += 1;
                if n == 1 {
                    c_swap = (c1, c2);
                } else if n == 2 && c_swap != (c2, c1) || n > 2 {
                    return false;
                }
            }
        }

        return n != 1;
    }
}

fn main() {
    println!("Hello, world!");
}
