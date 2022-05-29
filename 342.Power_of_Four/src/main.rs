struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let log = (n as f64).log(4.0);
        return n > 0 && log.round() == log;
    }
}

fn main() {
    println!("{:?}", Solution::is_power_of_four(16));
    println!("{:?}", Solution::is_power_of_four(5));
    println!("{:?}", Solution::is_power_of_four(1));
}
