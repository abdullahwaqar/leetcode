struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let log = (n as f64).log(3.0);
        return n > 0 && log == log;
    }
}

fn main() {
    println!("{:?}", Solution::is_power_of_three(243));
    println!("{:?}", Solution::is_power_of_three(45));
}
