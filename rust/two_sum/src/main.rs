use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(i32, i32)> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, &j) in nums.iter().enumerate() {
            let compliment = target - j;
            match map.get(&compliment) {
                Some(&index) => return Some((i as i32, index as i32)),
                None => {
                    map.insert(j, i);
                }
            }
        }
        None
    }
}
