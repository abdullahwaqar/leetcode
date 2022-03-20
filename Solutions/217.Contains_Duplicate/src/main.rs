struct Solution {}

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        // * using sort because taking Data Structure I
        nums.sort_unstable();

        //* index based for loop
        for idx in 1..nums.len() {
            // if current element is equal to previous element
            // sort will grantee that same element will be adjacent 
            if nums[idx -1] == nums[idx] {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    println!("Hello, world!");
}
