struct Solution {}

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, k: i32) -> Vec<i32> {
        // let buffer = num
        //     .iter()
        //     .map(|n| n.to_string())
        //     .collect::<Vec<String>>()
        //     .join("")
        //     .parse::<i64>()
        //     .unwrap()
        //     + k as i64;

        // return buffer
        //     .to_string()
        //     .chars()
        //     .into_iter()
        //     .map(|c| c.to_string().parse::<i32>().unwrap())
        //     .collect::<Vec<i32>>();

        // let mut carry = k;
        // let capacity = num.len() + {
        //     if k == 0 {
        //         k as usize
        //     } else {
        //         (k as f32).log(10.0).floor() as usize
        //     }
        // };
        // let mut res = Vec::with_capacity(capacity);
        // a.reverse();
        // let mut i = 0;
        // while i < a.len() || carry > 0 {
        //     let x = {
        //         if i >= a.len() {
        //             carry
        //         } else {
        //             a[i] + carry
        //         }
        //     };
        //     res.push(x % 10);
        //     carry = x / 10;
        //     i += 1;
        // }
        // res.reverse();
        // res
    }
}

fn main() {
    println!("{:?}", Solution::add_to_array_form(vec![1, 2, 0, 0], 34));
    println!("{:?}", Solution::add_to_array_form(vec![2, 1, 5], 806));
    println!(
        "{:?}",
        Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1)
    );
}
