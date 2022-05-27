use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();

        for idx in 0..nums.len() {
            if let Some(val) = map.get(&nums[idx]) {
                if idx - val <= k as usize {
                    return true;
                }
            }
            map.insert(nums[idx], idx);
        }
        return false;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
    );
}
