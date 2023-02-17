struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|num| num % 2);
        return nums;
    }
}

fn main() {
    println!("Hello, world!");
}
