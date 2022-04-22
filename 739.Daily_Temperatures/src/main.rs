struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answer_buffer = vec![0; temperatures.len()];
        let mut stack = Vec::new();

        for (idx, &temp) in temperatures.iter().enumerate().rev() {
            while let Some(&j) = stack.last() {
                if temp >= temperatures[j] {
                    stack.pop();
                } else {
                    break;
                }
            }

            answer_buffer[idx] = match stack.last() {
                Some(temp) => (temp - idx) as i32,
                None => 0,
            };

            stack.push(idx);
        }

        return answer_buffer;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    );
}
