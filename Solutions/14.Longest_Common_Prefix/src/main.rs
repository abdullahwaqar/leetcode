struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = strs.first().unwrap().clone();
        if strs.is_empty() {
            return "".to_string();
        };

        for i in 1..=strs.len() - 1 {
            println!("{:?}", i);
            for c in strs[i].chars() {
                for r_c in result.chars() {
                    if r_c != c {
                        println!("{:?}", r_c);
                        result.remove(result.chars().position(|x| x == r_c).unwrap());
                    }
                }
            }
        }

        return result;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::longest_common_prefix(
            Vec::from(["flower", "flow", "flight"])
                .iter()
                .map(|elem| elem.to_string())
                .collect()
        )
    );
}
