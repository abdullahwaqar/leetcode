struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        return nums.iter().fold(1, |acc, &val| acc * val.signum());
    }
}

fn main() {
    println!("Hello, world!");
}
