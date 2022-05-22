use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        // * Base case
        dp[0] = 0;
        // coins.sort_by(|a, b| b.cmp(a));
        println!("{:?}", dp);

        for amount in 0..dp.len() {
            for coin in &coins {
                let amount_left = amount as i32 - coin;
                if amount_left >= 0 {
                    dp[amount] = min(dp[amount], 1 + dp[amount_left as usize]);
                }
            }
        }

        return if dp[amount as usize] != amount + 1 {
            dp[amount as usize]
        } else {
            -1
        };
    }
}

fn main() {
    println!("{:#}", Solution::coin_change(vec![1, 2, 5], 11));
}
