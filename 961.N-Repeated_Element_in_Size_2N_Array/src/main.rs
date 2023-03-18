struct Solution {}

impl Solution {
    pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        println!("{:?}", nums);

        for idx in 0..nums.len() - 1 {
            if nums[idx] == nums[idx + 1] {
                return nums[idx];
            }
            println!("{:?}", nums[idx + 1]);
        }

        return 0;
    }
}

fn main() {
    println!("{:?}", Solution::repeated_n_times(vec![1, 2, 3, 3]));
    println!("{:?}", Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]));
}
