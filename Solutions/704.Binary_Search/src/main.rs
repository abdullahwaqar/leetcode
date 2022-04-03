use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        return -1;
    }

    // If you only want to see if element exists
    fn do_search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return -1;
        }

        let mid = nums.len() / 2;
        println!("Mid: {:?}", mid);
        let mid_value = nums[mid];
        if mid_value == target {
            return mid as i32;
        }

        println!("{:?}", nums);

        let (left, right) = nums.split_at(mid);
        println!("{:?} - {:?}", left, right);

        if mid_value < target {
            // Move to right
            println!("Going right");
            return Solution::search(right.to_vec(), target);
        } else {
            // Move to left
            println!("Going left");
            return Solution::search(left.to_vec(), target);
        }
    }
}

fn main() {
    println!("{:?}", Solution::search(vec![-1, 0, 3, 5, 9, 12, 14], 9));
}
