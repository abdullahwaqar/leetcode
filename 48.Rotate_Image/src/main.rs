struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // let matrix_len = matrix.len();
        //         // * Transpose
        //         for row_idx in 0..matrix_len {
        //             for col_idx in 0..matrix_len {
        //                 let curr_pixel = matrix[row_idx][col_idx];
        //                 matrix[row_idx][col_idx] = matrix[col_idx][row_idx];
        //                 matrix[col_idx][row_idx] = curr_pixel;
        //             }
        //         }

        //         // * Do a flip
        //         for row_idx in 0..matrix_len {
        //             for col_idx in 0..matrix_len / 2 {
        //                 let curr_pixel = matrix[row_idx][col_idx];
        //                 matrix[row_idx][col_idx] = matrix[row_idx][matrix_len - col_idx - 1];
        //                 matrix[row_idx][matrix_len - col_idx - 1] = curr_pixel;
        //             }
        //         }
        matrix.reverse();
        // * Transpose
        for row_idx in 0..matrix.len() {
            for col_idx in row_idx..matrix.len() {
                let x = matrix[row_idx][col_idx];
                let y = matrix[col_idx][row_idx];
                matrix[col_idx][row_idx] = x;
                matrix[row_idx][col_idx] = y;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
