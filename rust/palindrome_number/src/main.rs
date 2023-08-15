fn main() {
    println!("Hello, world!");
}
struct Solution{}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = match x >= 0 {
            true => x.to_string(),
            false => return false,
        };

        str.chars().rev().collect::<String>() == str
    }
}

#[test]
fn test_is_palindrome() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
}
