struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            println!("{:?}", digit);
            if digit < &mut 9 {
                *digit += 1;
                return digits;
            }
            *digit = 0;
        }

        let mut new_number = std::iter::repeat(0)
            .take(digits.len() + 1)
            .collect::<Vec<_>>();
        new_number[0] = 1;
        return new_number;
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
    println!("{:?}", Solution::plus_one(vec![9, 9, 9]));
}
