struct Solution {}

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (row, column) = (mat.len(), mat[0].len());

        // * If same size of matrix to reshape return the original
        if row * column != (r * c) as usize {
            return mat;
        }

        return mat.concat().chunks(c as usize).map(|r| r.to_vec()).collect();
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 3]], 1, 4)
    );
}
