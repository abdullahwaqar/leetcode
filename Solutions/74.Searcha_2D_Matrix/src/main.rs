struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }

        for row in matrix.iter() {
            for &column in row.iter() {
                if column == target {
                    return true;
                }
            }
        }

        return false;
    }
}

fn main() {
    println!("Hello, world!");
}
