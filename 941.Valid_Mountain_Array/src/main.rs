struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() <= 2 || arr[0] > arr[1] {
            return false;
        }

        let mut rising = true;
        let mut prev = arr[0];

        for val in &arr[1..] {
            match val.cmp(&prev) {
                std::cmp::Ordering::Less => rising = false,
                std::cmp::Ordering::Equal => return false,
                std::cmp::Ordering::Greater => {
                    if !rising {
                        return false;
                    }
                }
            }

            // * Update pointer
            prev = *val;
        }

        return !rising;
    }
}

fn main() {
    println!("{:?}", Solution::valid_mountain_array(vec![2, 1]));
    println!("{:?}", Solution::valid_mountain_array(vec![3, 5, 5]));
    println!("{:?}", Solution::valid_mountain_array(vec![0, 3, 2, 1]));
}
