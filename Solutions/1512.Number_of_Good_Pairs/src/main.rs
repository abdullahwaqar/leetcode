use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut pairs = HashMap::new();
        let mut count = 0;

        for num in nums {
            let value = pairs.entry(num).or_insert(0);
            count += *value;
            *value += 1;
        }

        return count;
    }
}

fn main() {
    println!("Hello, world!");
}
