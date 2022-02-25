struct Solution {}

impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        if nums1.len() < nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        nums1.sort_unstable();
        nums2.sort_unstable();

        println!("{:?}", nums1);

        let mut left = 0 as usize;
        let mut right = nums1.len() - 1;
        let mut middle = (left + right) / 2;
        println!("{:?}", middle);
        for num in nums2 {
            while left <= right {
                if nums1[middle] == num {
                    result.push(num);
                } else {
                    if nums1[middle] < num {
                        left = middle + 1;
                    } else {
                        right = middle - 1;
                    }
                }
                middle = (left + right) >> 1;
            }
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]));
}
