struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut decreasing = true;
        let mut increasing = true;

        for i in 0..nums.len() - 1 {
            decreasing &= nums[i] >= nums[i + 1];
            increasing &= nums[i] <= nums[i + 1];
        }

        return decreasing || increasing;
    }
}

fn main() {
    println!("{:?}", Solution::is_monotonic(vec![1, 2, 2, 3]));
}
