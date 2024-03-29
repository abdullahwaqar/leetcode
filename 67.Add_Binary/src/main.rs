struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        return format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        );
    }
}

fn main() {
    println!("Hello, world!");
}
