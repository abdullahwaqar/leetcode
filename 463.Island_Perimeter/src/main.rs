use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: i32, j: i32, grid: &Vec<Vec<i32>>, visited: &mut HashSet<(i32, i32)>) -> i32 {
            // * This is perimeter
            if i >= grid.len() as i32
                || j >= grid[0].len() as i32
                || i < 0
                || j < 0
                || grid[i as usize][j as usize] == 0
            {
                return 1;
            }

            // * Already visited
            if visited.contains(&(i, j)) {
                return 0;
            }

            visited.insert((i, j));
            // * 2d movement
            let mut perimter = dfs(i, j + 1, grid, visited);
            perimter += dfs(i + 1, j, grid, visited);
            perimter += dfs(i, j - 1, grid, visited);
            perimter += dfs(i - 1, j, grid, visited);
            return perimter;
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    return dfs(i as i32, j as i32, &grid, &mut visited);
                }
            }
        }

        return 0;
    }
}

fn main() {
    println!("Hello, world!");
}
