use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut max_str = 0;

        for right in 0..s.len() {
            while set.contains(&s[right as usize]) {
                set.remove(&s[left as usize]);
                left += 1;
            }
            set.insert(s[right as usize]);
            max_str = max_str.max(right - left + 1);
        }
        return max_str as i32;
    }
}

fn main() {
    println!("Hello, world!");
}
