struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut col = 0;
        let n = mat.len();

        return mat
            .iter()
            .map(|arr| {
                let row_sum = if col == n - col - 1 {
                    arr[col]
                } else {
                    arr[col] + arr[n - col - 1]
                };
                col += 1;
                return row_sum;
            })
            .sum::<i32>();
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}
