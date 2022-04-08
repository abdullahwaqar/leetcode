struct Solution {}

impl Solution {
    pub fn find_combination(
        candidates: &mut Vec<i32>,
        mut idx: i32,
        mut target: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        println!("{:#?}", current);
        if target == 0 {
            result.push(current.to_vec());
            println!("{:#?}", current);
            return;
        }

        for i in 0..candidates.len() {
            if candidates[i as usize] <= target {
                current.push(candidates[i as usize]);
                Solution::find_combination(
                    candidates,
                    i as i32,
                    target - candidates[i as usize],
                    current,
                    result,
                );
                current.clear();
            }
        }
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        Solution::find_combination(&mut candidates, 0, target, &mut Vec::new(), &mut result);

        return result;
    }
}

fn main() {
    println!("{:#?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
}
