struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut buffer = Vec::new();

        for num in nums.iter() {
            buffer.push(nums.iter().filter(|value| value.lt(&num)).count() as i32);
        }

        return buffer;
    }
}

fn main() {
    println!("Hello, world!");
}
