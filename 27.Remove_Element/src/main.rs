use core::num;

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut remove_count: i32 = 0;

        for idx in 0..nums.len() {
            if nums[idx] != val {
                nums[remove_count as usize] = nums[idx];
                remove_count += 1;
            }
        }

        return remove_count;
    }
}
fn main() {
    // println!("{:?}", Solution::remove_element(nums, val));
}
