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

fn is_anagram(a: &str, b: &str) -> bool {
    // O(n)
    let mut char_track = [0; 128];
    a.chars().for_each(|c| char_track[c as usize] += 1);
    b.chars().for_each(|c| char_track[c as usize] -= 1);
    char_track == [0; 128]
}

fn get_words() -> (String, String) {
    println!("Please input your guess.");

    let mut a = String::new();
    let mut b = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut a).expect("Failed to read line");
    a = a.trim().to_string();
    stdin.read_line(&mut b).expect("Failed to read line");
    b = b.trim().to_string();

    (a, b)
}

#[test]
fn test_is_anagram() {
    assert_eq!(is_anagram("anagram", "nagaram"), true);
    assert_eq!(is_anagram("qwerty", "ytrewq"), true);
    assert_eq!(is_anagram("anagram", "nagaram"), true);
    assert_eq!(is_anagram("rat", "car"), false);
    assert_eq!(is_anagram("rat", "car"), false);
}
