struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        return grid.iter().flatten().filter(|&val| val < &0).count() as i32;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ])
    );
}
