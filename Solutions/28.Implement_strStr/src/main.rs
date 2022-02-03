struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // if needle.is_empty() {
        //     return 0;
        // }

        // let needle_char_buffer = needle.chars().nth(0).unwrap();

        // for i in 0..haystack.len() {
        //     if haystack.chars().nth(i).unwrap() == needle_char_buffer && i != 0 {
        //         return i as i32;
        //     }
        // }

        // return -1;

        if needle.is_empty() {
            return 0;
        }
        
        match &haystack.find(&needle) {
            None => -1,
            Some(idx) => *idx as i32,
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::str_str(String::from("hello"), String::from("ll"))
    );
    println!(
        "{:?}",
        Solution::str_str(String::from(""), String::from("ll"))
    );
    println!(
        "{:?}",
        Solution::str_str(String::from("aaaaa"), String::from("bba"))
    );
    println!(
        "{:?}",
        Solution::str_str(String::from("aaa"), String::from("aaaa"))
    );
}
