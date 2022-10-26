struct Solution {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        return word1.join("").to_owned().eq(&word2.join("").to_owned());
    }
}

fn main() {
    println!(
        "{:#?}",
        Solution::array_strings_are_equal(
            vec!["ab".to_owned(), "c".to_owned()],
            vec!["a".to_owned(), "bc".to_owned()]
        )
    );
}
