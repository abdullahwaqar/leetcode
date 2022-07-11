struct Solution {}

impl Solution {
    pub fn license_key_formatting(mut s: String, k: i32) -> String {
        s = s.replace("-", "");
        let mut sink_size = match s.len() % k as usize {
            0 => k,
            rem => rem as i32,
        };

        return s.chars().into_iter().fold(String::new(), |mut acc, _char| {
            if sink_size == 0 {
                acc.push('-');
                sink_size = k;
            }
            acc.push(_char);
            sink_size -= 1;
            return acc;
        }).to_ascii_uppercase();
    }
}

fn main() {
    println!(
        "{:#?}",
        Solution::license_key_formatting("5F3Z-2e-9-w".to_owned(), 4)
    );
    println!(
        "{:#?}",
        Solution::license_key_formatting("2-5g-3-J".to_owned(), 2)
    );
}
