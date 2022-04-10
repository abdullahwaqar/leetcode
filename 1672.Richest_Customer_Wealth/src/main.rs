struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        return accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}
