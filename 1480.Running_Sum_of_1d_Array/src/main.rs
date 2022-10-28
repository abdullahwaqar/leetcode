struct Solution {}

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for idx in 1..nums.len() {
            nums[idx] += nums[idx -1];
        }

        return nums;
    }
}

fn main() {
    println!("{:?}", Solution::running_sum(vec![1, 2, 3, 4]));
}
