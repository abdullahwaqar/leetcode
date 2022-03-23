use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut prev_value_cache = HashMap::new();

        for (idx, val) in nums.iter().enumerate() {
            match prev_value_cache.get(&(target - *val)) {
                Some(i) => return vec![idx as i32, *i],
                None => prev_value_cache.insert(*val, idx as i32),
            };
        }
        unreachable!();
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
