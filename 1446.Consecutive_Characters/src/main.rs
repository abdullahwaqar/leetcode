struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let (mut max_char_count, mut res) = (1, 0);
        let mut prev_char = ' ';

        for cur_char in s.chars() {
            if cur_char == prev_char {
                res += 1;
                max_char_count = max_char_count.max(res);
            } else {
                // * Reset and update prev char pointer
                res = 1;
                prev_char = cur_char;
            }
        }
        return max_char_count;
    }
}

fn main() {
    println!("{:?}", Solution::max_power("leetcode".to_owned()));
    println!(
        "{:?}",
        Solution::max_power("abbcccddddeeeeedcba".to_owned())
    );
}
