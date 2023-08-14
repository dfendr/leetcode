use std::collections::{HashSet, HashMap};

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, j) in nums.iter().enumerate() {
            let compliment = target - j;
            if map.contains_key(&compliment) {
                return vec![i as i32, *map.get(&compliment).unwrap() as i32];
            }
            else {
                map.insert(*j, i);
            }
        }
        vec![]
    }
}
