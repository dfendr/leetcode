/*
Goal: Produce bool if two input words are anagrams or not.
*/

// Take in two strings

// Sort copies of them

use std::io;

// Print true/false if sorts are identical
fn main() {
    println!("{}", is_anagram("anagrassm", "anagarm"));
}

fn is_anagram_slow(a: &str, b: &str) -> bool {
    // O(log(n))
    let mut a: Vec<char> = a.chars().collect();
    a.sort();
    let mut b: Vec<char> = b.chars().collect();
    b.sort();
    a == b
}

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        //O(n)
        let mut char_track = [0; 128];
        s.chars().for_each(|c| char_track[c as usize] += 1);
        t.chars().for_each(|c| char_track[c as usize] -= 1);
        char_track == [0; 128]
    }
}

#[test]
fn test_is_anagram() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("qwerty".to_string(), "ytrewq".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}
