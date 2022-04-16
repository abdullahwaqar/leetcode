struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_vec = s.chars().collect::<Vec<char>>();
        let mut t_vec = t.chars().collect::<Vec<char>>();
        s_vec.sort_unstable();
        t_vec.sort_unstable();
        return s_vec == t_vec;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::is_anagram("rat".to_owned(), "car".to_owned())
    );
}
