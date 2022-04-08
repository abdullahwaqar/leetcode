struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut pos = 0;

        for idx in 0..nums.len() {
            if nums[idx] == target {
                pos = idx as i32;
            }

            if nums[idx] < target {
                pos = idx as i32 + 1;
            }
        }

        return pos;
    }
}

fn main() {
    println!("Hello, world!");
}
