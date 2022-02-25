struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result_buff = vec![vec![0; matrix.len()]; matrix[0].len()];

        for (i, row) in matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                result_buff[j][i] = *cell;
            }
        }

        return result_buff;
    }
}

fn main() {
    println!("Hello, world!");
}
