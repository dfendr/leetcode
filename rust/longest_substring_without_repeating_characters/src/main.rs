use std::collections::HashSet;

fn main() {
    let n = Solution::length_of_longest_substring("aa".to_string());
    println!("{n}");
}

struct Solution {}
impl Solution {
    pub fn length_of_longest_substring_naive(s: String) -> i32 {
        if s.is_empty() { return 0; } let (mut max , mut start, stop) = (0, 0, s.len()); let mut char_set = HashSet::new(); while stop as i32 - start as i32 != -1 { for c in s[start..=stop].as_bytes() { if !char_set.insert(c) { break; } } let substr_len = char_set.len(); if substr_len == s.len() { return s.len() as i32; } else if substr_len > max { max = substr_len; } char_set.drain(); start += 1; } max as i32
    }

    fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() { return 0; }
        let (mut max, mut start, mut end) = (0, 0, 0);
        let s_bytes = s.as_bytes();
        let mut char_set = HashSet::new();
        while end < s.len() {
            if char_set.insert(s_bytes[end] as char) {
                end += 1;
                max = std::cmp::max(max, char_set.len());
            } else {
                char_set.remove(&(s_bytes[start] as char));
                start += 1;
            }
        }
        max as i32
    }
}

#[cfg(test)]
#[test]
fn test1() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}
#[test]
fn test2() {
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
}
#[test]
fn test3() {
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}
#[test]
fn test4() {
    assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
}
#[test]
fn test5() {
    assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
}
#[test]
fn test6() {
    assert_eq!(Solution::length_of_longest_substring("aa".to_string()), 1);
}
