struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left_sum = 0;

        println!("{}", sum);

        for (i, &val) in nums.iter().enumerate() {
            if sum - left_sum - val == left_sum {
                return i as i32;
            }
            left_sum += val;
        }

        return -1;
    }
}

fn main() {
    println!("{:?}", Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
}
