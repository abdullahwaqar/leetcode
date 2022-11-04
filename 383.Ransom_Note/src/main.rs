struct Solution {}

// https://leetcode.com/problems/ransom-note/solutions/1952584/rust-solution/
// impl Solution {
//     pub fn can_construct(ransom_note: String, magazine: String) -> bool {
//         if ransom_note.len() > magazine.len() {
//             return false;
//         }

//         let mut map = [0; 26];
//         let mut flags = 0;
//         let idx = |c: char| c as usize - 'a' as usize;

//         ransom_note.chars().for_each(|c| {
//             let i = idx(c);
//             map[i] += 1;
//             flags |= 1 << i; // set flag
//         });

//         for c in magazine.chars() {
//             let i = idx(c);
//             map[i] -= 1;
//             if map[i] == 0 {
//                 flags ^= 1 << i;  // drop flag
//                 if flags == 0 {
//                     return true;
//                 }
//             }
//         }
//         false
//     }
// }

impl Solution {
    fn set_difference<T: Ord + Clone>(mut slice: &[T], mut remove: &[T]) -> Vec<T> {
        let mut out = Vec::new();
        while let (Some(sf), Some(rf)) = (slice.first(), remove.first()) {
            if sf == rf {
                slice = &slice[1..];
                remove = &remove[1..];
            } else if sf < rf {
                out.push(sf.clone());
                slice = &slice[1..];
            } else {
                remove = &remove[1..];
            }
        }
        out
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut d: [usize; 256] = [0; 256];
        for c in magazine.chars().map(|c| (c as u8) as usize) {
            d[c] += 1;
        }

        for c in ransom_note.chars().map(|c| (c as u8) as usize) {
            if d[c] == 0 {
                return false;
            }
            d[c] -= 1;
        }
        return true;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::can_construct("aa".to_owned(), "ab".to_owned())
    );
}
