struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        let mut last_idx = nums1.len() - 1;
        m -= 1;
        n -= 1;

        while last_idx as i32 >= 0 {
            println!("{:?}", last_idx);
            if m < 0 {
                nums1[last_idx] = nums2[n as usize];
                n -= 1;
            } else if n < 0 {
                nums1[last_idx] = nums1[m as usize];
                m -= 1;
            } else {
                if nums1[m as usize] > nums2[n as usize] {
                    nums1[last_idx] = nums1[m as usize];
                    m -= 1;
                } else {
                    nums1[last_idx] = nums2[n as usize];
                    n -= 1;
                }
            }

            last_idx -= 1;
        }

        println!("{:?}", nums1);
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3)
    );
    println!(
        "{:?}",
        Solution::merge(&mut vec![0], 0, &mut vec![1], 1)
    );
}
