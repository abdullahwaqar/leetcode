use rand::{distributions::Uniform, prelude::Distribution};

struct Solution {
    x_center: f64,
    y_center: f64,
    radius: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(-self.radius..self.radius);

        loop {
            let (x, y) = (range.sample(&mut rng), range.sample(&mut rng));
            if x.powi(2) + y.powi(2) < self.radius.powi(2) {
                break vec![self.x_center + x, self.y_center + y];
            }
        }
    }
}

fn main() {
    let obj = Solution::new(1.0, 0.0, 0.0);
    let ret_1: Vec<f64> = obj.rand_point();
    println!("{:?}", ret_1);
}
