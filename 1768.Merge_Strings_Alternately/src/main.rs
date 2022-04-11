struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result: String = "".to_string();
        for (char1, char2) in word1.chars().zip(word2.chars()) {
            result.push(char1);
            result.push(char2);
        }

        if word1.len() < word2.len() {
            // Add rest of the word2
            result.extend(&mut word2[word1.len()..word2.len()].chars());
        } else {
            // Add rest of the word1
            result.extend(&mut word1[word2.len()..word1.len()].chars());
        }
        return result;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::merge_alternately("ab".to_owned(), "pqrs".to_owned())
    );
}
