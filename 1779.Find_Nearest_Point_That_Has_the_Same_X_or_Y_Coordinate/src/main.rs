struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min_dist = std::i32::MAX;
        let mut return_point = -1;

        for (idx, point) in points.iter().enumerate() {
            if x == point[0] || y == point[1] {
                let curr_distance = (x - point[0]).abs() + (y - point[1]).abs();
                if curr_distance < min_dist || return_point == -1 && curr_distance == curr_distance
                {
                    return_point = idx as i32;
                    min_dist = curr_distance;
                    if min_dist == 0 {
                        break;
                    }
                }
            }
        }

        return return_point;
    }
}

fn main() {
    println!("Hello, world!");
}
