struct Solution {}

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        let (mut res, mut base) = (0, 1);

        while num != 0 {
            println!("Before {} - Result {}", num, res);
            // https://www.purplemath.com/modules/numbbase2.htm
            res += base * (num % 7);
            base *= 10;
            num /= 7;
            println!("After {} - Result {}", num, res);
        }

        return res.to_string();
    }
}

fn main() {
    println!("{}", Solution::convert_to_base7(100));
}
