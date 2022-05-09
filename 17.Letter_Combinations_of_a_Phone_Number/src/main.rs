struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut digits = digits.chars();
        let letters = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];

        for digit in digits {
            let _char = letters[digit.to_digit(10).unwrap() as usize]
                .chars()
                .collect::<>();
            println!("{:?}", _char);
        }

        return vec![];
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_owned()));
}
