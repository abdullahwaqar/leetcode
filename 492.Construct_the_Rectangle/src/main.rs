struct Solution {}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        // The W is always less than or equal to the square root
        // of the area, so we start searching at sqrt(area) till we find the result.

        let mut height = 1;
        let mut width = 1;

        for breadth in 1..=(area as f32).sqrt() as u32 {
            if area % breadth as i32 == 0 {
                height = area / breadth as i32;
                width = breadth as i32;
            }
        }

        return vec![height, width];
    }
}

fn main() {
    let answer = Solution::construct_rectangle(4);
    println!("{:?}", answer);
}
