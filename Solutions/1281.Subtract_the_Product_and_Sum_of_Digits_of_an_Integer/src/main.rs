struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut buffer = vec![];
        while n > 0 {
            buffer.push(n % 10);
            n /= 10;
        }

        return buffer.iter().fold(1, |acc, digit| acc * digit) - buffer.iter().sum::<i32>();
    }
}

fn main() {
    println!("Hello, world!");
}
