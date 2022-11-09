use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = Vec::with_capacity(cost.len());

        for n in 0..cost.len() {
            match n {
                0 => dp.push(cost[n]),
                1 => dp.push(cost[n]),
                _ => dp.push(cost[n] + min(dp[n - 1], dp[n - 2])),
            };
        }
        return min(dp[dp.len() - 1], dp[dp.len() - 2]);
    }
}

fn main() {
    println!("Hello, world!");
}
