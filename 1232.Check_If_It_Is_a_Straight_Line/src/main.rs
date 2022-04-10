struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // coordinates.iter().enumerate().all(|(idx, val)| val[0] + 1 == v);
        let x_delta = coordinates[1][0] - coordinates[0][0];
        let y_delta = coordinates[1][1] - coordinates[0][1];
        println!("x delta: {:?} y delta: {}", x_delta, y_delta);

        // Slope = Y2 - Y1 / X2 - X1
        for idx in 0..coordinates.len() - 1 {
            let temporal_x = coordinates[idx + 1][0] - coordinates[idx][0];
            let temporal_y = coordinates[idx + 1][1] - coordinates[idx][1];
            if x_delta * temporal_y != y_delta * temporal_x {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ])
    );
    println!(
        "{:?}",
        Solution::check_straight_line(vec![vec![0, 0], vec![0, 1], vec![0, -1],])
    );
}
