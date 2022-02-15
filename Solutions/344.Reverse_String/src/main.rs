struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut right = s.len() - 1;
        let mut left = 0 as usize;

        while left < right {
            s.swap(left, right);
            right -= 1;
            left += 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
