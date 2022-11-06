struct Solution {}

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn fill(image: &mut Vec<Vec<i32>>, color: i32, sr: usize, sc: usize, new_color: i32) {
            let (row, col) = (image.len(), image[0].len());

            if !(sr < 0 as usize
                || sc < 0 as usize
                || sc >= col
                || sr >= row
                || image[sr][sc] != color)
            {
                image[sr][sc] = new_color;

                fill(image, color, sr - 1, sc, new_color);
                fill(image, color, sr + 1, sc, new_color);
                fill(image, color, sr, sc, new_color);
                fill(image, color, sr, sc - 1, new_color);
                fill(image, color, sr, sc + 1, new_color);
            }
        }

        let (sr, sc) = (sr as usize, sc as usize);

        let old_color = image[sr][sc];

        if image[sr][sc] != new_color {
            fill(&mut image, old_color, sr, sc, new_color);
        }

        return image;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
    );
}
