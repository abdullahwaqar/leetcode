struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut sqr_vec = nums
            .iter()
            .map(|element| element * element)
            .collect::<Vec<i32>>();
        sqr_vec.sort_unstable();
        return sqr_vec;
    }
}

fn main() {
    println!("{:?}", Solution::sorted_squares(vec![-7, -3, 2, 3, 11]));
}
