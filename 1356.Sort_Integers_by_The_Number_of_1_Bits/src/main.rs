struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));
        return arr;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
    );
}
