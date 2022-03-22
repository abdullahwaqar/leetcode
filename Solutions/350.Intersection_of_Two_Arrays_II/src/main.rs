struct Solution {}

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut output_buffer: Vec<i32> = Vec::new();

        // Two pointers
        let mut num1_ptr = 0;
        let mut num2_ptr = 0;

        while num1_ptr < nums1.len() && num2_ptr < nums2.len() {
            if nums1[num1_ptr] == nums2[num2_ptr] {
                // common element found add to out buffer and increment both pointers
                output_buffer.push(nums1[num1_ptr]);
                num1_ptr += 1;
                num2_ptr += 1;
            } else if nums1[num1_ptr] < nums2[num2_ptr] {
                num1_ptr += 1;
            } else if nums1[num1_ptr] > nums2[num2_ptr] {
                num2_ptr += 1;
            }
        }

        return output_buffer;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
    );
}
