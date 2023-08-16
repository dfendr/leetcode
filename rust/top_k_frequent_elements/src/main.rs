use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    Solution::top_k_frequent(vec![1, 2, 3, 4], 2);
}

struct Solution {}
impl Solution {
    pub fn top_k_frequent_naive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in nums.iter() {
            map.entry(*i).and_modify(|n| *n += 1).or_insert(1);
        }
        let mut vec: Vec<_> = map.iter().collect();
        vec.sort_by(|a, b| b.1.cmp(a.1));
        vec.iter().take(k as usize).map(|(k, _)| **k).collect()
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); nums.len()+1];

        let mut map: HashMap<i32, usize> = HashMap::new();
        for &i in &nums {
            *map.entry(i).or_insert(1) += 1;
        }

        for (&k, &v) in &map {
            bucket[v].push(k);
        }

        let mut res = Vec::new();
        for vec in bucket.iter().rev() {
            for &i in vec {
                if res.len() == k as usize {
                    return res;
                }
                res.push(i)
            }
        }
        res
    }
}
