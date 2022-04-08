struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        return (x ^ y).count_ones() as _;
    }
}

fn main() {
    println!("{:?}", Solution::hamming_distance(1, 4));
}
