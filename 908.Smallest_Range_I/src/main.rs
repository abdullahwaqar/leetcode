use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (mut min_val, mut max_val) = (nums[0], nums[0]);

        for num in nums.iter().skip(1) {
            min_val = min(min_val, *num);
            max_val = max(max_val, *num);
        }

        return if min_val + k >= max_val - k {
            0
        } else {
            (max_val - k) - (min_val + k)
        };
    }
}

fn main() {
    println!("Hello, world!");
}
