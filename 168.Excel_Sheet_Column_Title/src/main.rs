struct Solution {}

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let genesis_char = 'A' as u8;
        let mut result = String::new();

        while column_number > 0 {
            // Z -> 26
            let current_char_idx = ((column_number - 1) % 26) as u8;

            println!("{:?}", current_char_idx);

            // * Get current char
            let current_char = (genesis_char + current_char_idx) as char;
            println!("{:?}", current_char);

            result.push(current_char);

            column_number = (column_number - 1) / 26;
        }

        return result.chars().rev().collect();
    }
}

fn main() {
    println!("{:?}", Solution::convert_to_title(27));
}
