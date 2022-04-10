struct Solution {}

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for idx in (1..=arr.len()).step_by(2) {
            for window in arr.windows(idx) {
                result += window.iter().sum::<i32>();
            }
        }
        return result;
    }
}

fn main() {
    println!("Hello, world!");
}
