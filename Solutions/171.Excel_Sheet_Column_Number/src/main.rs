struct Solution {}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        // for col in column_title.into_bytes() {
        //     println!("{:?}", col as i32 - ('A' as i32) + 1);
        // }

        return column_title
            .chars()
            .map(|c| c as i32 - ('A' as i32) + 1)
            .fold(0, |previous, next| previous * 26 + next);
    }
}

fn main() {
    println!("{:?}", Solution::title_to_number("A".to_string()));
    println!("{:?}", Solution::title_to_number("B".to_string()));
}
