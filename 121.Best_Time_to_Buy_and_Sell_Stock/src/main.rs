use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        let mut buy = 0;
        let mut sell = 1;

        while sell < prices.len() {
            if prices[buy] < prices[sell] {
                let current_profit = prices[sell] - prices[buy];
                profit = cmp::max(current_profit, profit);
            } else {
                // * Update as buying because at this point this is the lowest buying
                buy = sell;
            }
            sell += 1;
        }

        return profit;
    }
}
fn main() {
    println!("Hello, world!");
}
