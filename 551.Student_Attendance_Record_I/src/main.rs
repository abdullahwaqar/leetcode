struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        if s.contains("LLL") {
            return false;
        }

        return if s.chars().filter(|c| c == &'A').count() > 1 {
            false
        } else {
            true
        };
    }
}

fn main() {
    println!("{:?}", Solution::check_record("PPALLP".to_owned()));
    println!("{:?}", Solution::check_record("PPALLL".to_owned()));
}
