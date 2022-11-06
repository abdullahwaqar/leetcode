struct Solution {}

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k >= 2 {
            let mut str_buffer = s.chars().collect::<Vec<char>>();
            str_buffer.sort();
            return str_buffer.iter().collect::<String>();
        } else {
            let s = s.as_bytes();
            let mut buf = vec![0; s.len() + s.len()];

            buf[..s.len()].copy_from_slice(&s[..]);
            buf[s.len()..].copy_from_slice(&s[..]);

            return buf
                .windows(s.len())
                .skip(1)
                .min()
                .map(|ss| String::from_utf8(ss.to_vec()).unwrap())
                .unwrap();
        }
    }
}

fn main() {
    println!("{:?}", Solution::orderly_queue("baaca".to_owned(), 3));
    println!("{:?}", Solution::orderly_queue("tk".to_owned(), 1));
}
