struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // let mut sink = vec![];

        // nums.retain(|&num| {
        //     if num != 0 {
        //         return true;
        //     } else {
        //         sink.push(0);
        //         return false;
        //     }
        // });

        // nums.append(&mut sink);

        let mut i = 0;

        for _ in 0..nums.len() {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
