struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));

        // beauty
        match nums.windows(3).find(|l| l[0] < l[1] + l[2]) {
            Some(l) => l.iter().sum(),
            None => 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
