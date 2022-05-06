use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    max_heap: BinaryHeap<Reverse<i32>>,
    min_heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn len(&self) -> usize {
        self.min_heap.len() + self.max_heap.len()
    }

    fn add_num(&mut self, num: i32) {
        self.min_heap.push(num);

        let n = self.min_heap.pop().unwrap();
        self.max_heap.push(Reverse(n));

        let s_len = self.min_heap.len();
        let mid = self.len() / 2;

        if s_len <= mid {
            let Reverse(n) = self.max_heap.pop().unwrap();
            self.min_heap.push(n);
        }
    }

    fn find_median(&mut self) -> f64 {
        let len = self.len();

        let mut elems = vec![];

        if len % 2 == 0 {
            elems.push(self.min_heap.pop().unwrap());
            elems.push(self.min_heap.pop().unwrap());
        } else {
            elems.push(self.min_heap.pop().unwrap());
        }

        for &n in elems.iter() {
            self.min_heap.push(n);
        }

        elems.iter().sum::<i32>() as f64 / elems.len() as f64
    }
}

fn main() {
    //  Your MedianFinder object will be instantiated and called as such:
    let mut obj = MedianFinder::new();
    obj.add_num(3);
    obj.add_num(10);
    obj.add_num(11);
    let ret_2: f64 = obj.find_median();
    println!("{}", ret_2);
}
