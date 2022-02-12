struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let v = nums.clone();
        return [nums, v].concat();
    }
}
fn main() {
    println!("Hello, world!");
}
