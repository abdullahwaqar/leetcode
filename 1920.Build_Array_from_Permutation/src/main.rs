struct Solution {}

// impl Solution {
//     pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
//         let mask = 1023;
//         (0..nums.len()).for_each(|i| nums[i] |= (nums[nums[i] as usize] & mask) << 10);
//         (0..nums.len()).for_each(|i| nums[i] >>= 10);
//         nums
//     }
// }

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut buffer: Vec<i32> = Vec::with_capacity(nums.len());
        println!("{:?}", buffer.len());

        for elem in nums.iter() {
            buffer.push(nums[*elem as usize]);
        }

        return buffer;
    }
}

fn main() {
    println!("Hello, world!");
}
