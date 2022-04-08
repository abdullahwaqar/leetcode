struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut sentence_len = -1;

        sentences.iter().for_each(|sentence| {
            let sentence_words = sentence.split_whitespace().collect::<Vec<_>>().len() as i32;
            if sentence_len < sentence_words {
                sentence_len = sentence_words;
            }
        });

        return sentence_len;
    }
}

fn main() {
    println!("Hello, world!");
}
