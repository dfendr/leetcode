use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = [0; 26];
            s.chars()
                .for_each(|c| key[(c as usize) - ('a' as usize)] += 1);
            map.entry(key).or_insert_with(Vec::new).push(s);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]]
        );

        assert_eq!(
            Solution::group_anagrams(vec!["".to_string(), "".to_string()]),
            vec![vec!["", ""]]
        );

        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string()]),
            vec![vec!["a"]]
        );

        assert_eq!(Solution::group_anagrams(vec![]), Vec::<Vec<String>>::new());
    }
}
