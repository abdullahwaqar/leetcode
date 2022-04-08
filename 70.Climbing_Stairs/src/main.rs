struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut first_step = 1;
        let mut second_step = 2;

        for _ in 3..=n {
            let next_step = first_step + second_step;
            first_step = second_step;
            second_step = next_step;
        }

        return second_step;
    }
}

fn main() {
    println!("Hello, world!");
}
