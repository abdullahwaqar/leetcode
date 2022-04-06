struct Solution {}

impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort();
        salary.remove(0);
        salary.remove(salary.len() - 1);

        let salary_sum: i32 = salary.iter().sum();
        return salary_sum as f64 / salary.len() as f64;
    }
}

fn main() {
    println!("{:?}", Solution::average(vec![4000, 3000, 1000, 2000]));
    println!("{:?}", Solution::average(vec![1000, 2000, 3000]));
}
