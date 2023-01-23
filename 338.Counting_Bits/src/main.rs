struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        return (0..=n).map(|i| i.count_ones() as i32).collect();
    }
}

fn main() {
    println!("Hello, world!");
}
