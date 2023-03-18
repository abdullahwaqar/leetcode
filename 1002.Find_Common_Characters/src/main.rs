use std::{collections::HashMap, ops::Add};

struct Solution {}

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        // let mut lookup_table: HashMap<char, usize> = HashMap::new();
        // let max_len = words.len();

        // for word in words {
        //     let mut word_table: HashMap<char, usize> = HashMap::new();

        //     for ele in word.chars() {
        //         *word_table.entry(ele).or_insert(0) += 1;
        //     }

        //     for (key, val) in word_table {
        //         *lookup_table.entry(key).or_insert(0) += val;
        //     }
        // }

        // println!("{:?}", lookup_table);

        // let result = lookup_table
        //     .into_iter()
        //     .filter(|(_, val)| *val >= max_len)
        //     .map(|(key, _)| key.to_string())
        //     .collect::<Vec<_>>();
        // println!("{:?}", result);

        // return result;

        let mut common_chars = Vec::new();

        let mut char_count = [std::usize::MAX; 26];

        for word in &words {
            let mut word_count = [0; 26];

            for c in word.chars() {
                let idx = (c as u8 - b'a') as usize;
                word_count[idx] += 1;
            }

            for i in 0..26 {
                char_count[i] = std::cmp::min(char_count[i], word_count[i]);
            }
        }

        for i in 0..26 {
            for _ in 0..char_count[i] {
                let c = (b'a' + i as u8) as char;
                common_chars.push(c.to_string());
            }
        }

        return common_chars;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::common_chars(vec![
            "bella".to_owned(),
            "label".to_owned(),
            "roller".to_owned()
        ])
    );

    println!(
        "{:?}",
        Solution::common_chars(vec![
            "cool".to_string(),
            "lock".to_string(),
            "cook".to_string()
        ])
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let words = vec!["cool".to_string(), "lock".to_string(), "cook".to_string()];
        assert_eq!(
            Solution::common_chars(words),
            vec!["o".to_string(), "c".to_string()]
        );
    }
}
