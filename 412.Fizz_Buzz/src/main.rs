struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = vec![];
        for idx in 1..=n {
            let m1 = idx % 3 == 0;
            let m2 = idx % 5 == 0;

            result.push(match (m1, m2) {
                (true, true) => "FizzBuzz".to_owned(),
                (true, false) => "Fizz".to_owned(),
                (false, true) => "Buzz".to_owned(),
                _ => idx.to_string(),
            });
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::fizz_buzz(3));
}
