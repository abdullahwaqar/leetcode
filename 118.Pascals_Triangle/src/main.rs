struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut return_buffer: Vec<Vec<i32>> = Vec::new();

        for idx in 0..num_rows as usize {
            let current_vec_size = 1..idx;
            return_buffer.push(current_vec_size.fold(vec![1; idx + 1], |mut acc, i| {
                acc[i] = return_buffer[idx - 1][i - 1] + return_buffer[idx - 1][i];
                return acc;
            }));
        }

        return return_buffer;
    }
}

fn main() {
    println!("{:?}", Solution::generate(5));
}
