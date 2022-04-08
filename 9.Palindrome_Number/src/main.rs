struct Solution {}

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        // * If double digit or negative
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut rev_num = 0;

        // * Get the half of the number to rev_num and compare x and rev_num
        while x > rev_num {
            rev_num = (rev_num * 10) + x % 10;
            // Remove last digit
            x = x / 10;
        }

        return x == rev_num || x == rev_num / 10;
    }
}

fn main() {
    println!("{:?}", Solution::is_palindrome(121));
    println!("{:?}", Solution::is_palindrome(-121));
    println!("{:?}", Solution::is_palindrome(10));
    println!("{:?}", Solution::is_palindrome(0));
}
