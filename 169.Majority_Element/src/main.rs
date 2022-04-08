use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut num_entries = HashMap::new();

        for num in &nums {
            let count = num_entries.entry(num).or_insert(0);
            *count += 1;

            if *count > nums.len() / 2 {
                return *num;
            }
        }

        return 0;
    }
}

fn main() {
    println!("Hello, world!");
}
