struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut history_max_count = 0;
        let mut max_count = 0;

        nums.iter().for_each(|num| {
            if num.eq(&1) {
                max_count += 1;
            } else if num.eq(&0) {
                history_max_count = std::cmp::max(max_count, history_max_count);
                max_count = 0;
            }
        });
        return std::cmp::max(max_count, history_max_count);
    }
}

fn main() {
    println!(
        "{:#?}",
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );
    println!(
        "{:#?}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
    );
}
