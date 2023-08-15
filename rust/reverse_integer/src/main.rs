fn main() {
    println!("Hello, world!");
}
//CONSTRAINT: Assume

struct Solution {}

impl Solution {
    pub fn reverse(n: i32) -> i32 {
        let mut negative = false;

        let str = match n > 0 {
            true => n.to_string(),
            false => {
                negative = true;
                (-n).to_string()
            }
        };

        // Crashes on overflow
        let mut rev = str
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or_default();

        if negative {
            rev = -rev;
        }

        rev
    }
}
#[test]
fn test_reverse() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
}
