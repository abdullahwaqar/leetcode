struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max_value_1, mut max_value_2) = (0, 0);

        // * Naive
        for &num in nums.iter() {
            if num > max_value_2 {
                max_value_1 = std::mem::replace(&mut max_value_2, num);
            } else if num > max_value_1 {
                max_value_1 = num;
            }
        }

        return (max_value_1 - 1) * (max_value_2 - 1);
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![3, 4, 5, 2]));
    println!("{}", Solution::max_product(vec![1, 5, 4, 5]));
}
