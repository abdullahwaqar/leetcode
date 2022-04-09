struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        return arr
            .iter()
            .enumerate()
            .skip(1)
            .all(|(idx, val)| val - arr[idx - 1] == arr[1] - arr[0]);
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::can_make_arithmetic_progression(vec![3, 5, 1])
    );
}
