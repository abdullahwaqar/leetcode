struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut buffer = vec![];

        for idx_num1 in 0..nums1.len() {
            let mut idx = 0;
            for idx_num2 in 0..nums2.len() {
                if nums1[idx_num1] == nums2[idx_num2] {
                    idx = idx_num2;
                    break;
                }
            }

            let mut res = -1;

            for idx_num2 in idx..nums2.len() {
                if nums2[idx_num2] > nums2[idx] {
                    res = nums2[idx_num2];
                    break;
                }
            }
            buffer.push(res);
        }

        return buffer;
    }
}

fn main() {
    println!("Hello, world!");
}
