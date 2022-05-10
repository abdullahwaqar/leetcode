struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec![];

        if digits.is_empty() {
            return result;
        }

        let letters = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];

        for digit in digits.chars() {
            let chars = letters[digit.to_digit(10).unwrap() as usize].chars();

            if result.is_empty() {
                result = chars.into_iter().map(|c| c.to_string()).collect();
            } else {
                // fucking slow
                result = result
                    .iter()
                    .flat_map(|elem| {
                        chars
                            .clone()
                            .into_iter()
                            .map(|c| {
                                let mut res = elem.clone();
                                res.push(c.clone());
                                res
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect()
            }
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_owned()));
}
