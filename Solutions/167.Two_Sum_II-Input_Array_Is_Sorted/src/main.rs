struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() as i32 - 1;

        while left < right {
            let curr_sum = numbers[left as usize] + numbers[right as usize];

            if curr_sum > target {
                right -= 1;
            } else if curr_sum < target {
                left += 1;
            } else {
                // Found it
                return vec![left + 1, right + 1];
            }
        }
        unreachable!();
    }
}

fn main() {
    println!("Hello, world!");
}
