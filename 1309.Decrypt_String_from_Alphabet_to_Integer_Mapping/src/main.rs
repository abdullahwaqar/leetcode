use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut value_store = HashMap::new();
        let mut result = String::new();

        (0..26).into_iter().for_each(|idx| {
            value_store.insert(
                idx + 1,
                char::from_u32((97 + idx) as u32).unwrap().to_string(),
            );
        });

        let mut idx = 0;

        while idx < s.len() {
            if s.chars().nth(idx + 2) == Some('#') {
                result.push_str(
                    value_store
                        .get(&s[idx..idx + 2].parse::<i32>().unwrap())
                        .unwrap(),
                );
                idx = idx + 2;
            } else if s.chars().nth(idx).is_some() {
                let key = s
                    .chars()
                    .nth(idx)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                result.push_str(value_store.get(&key).unwrap());
            }
            idx += 1;
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::freq_alphabets("10#11#12".to_owned()));
}
