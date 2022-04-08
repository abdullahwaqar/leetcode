struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let buffer = nums.pop().unwrap();
            nums.insert(0, buffer);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
