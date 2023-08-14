fn main() {
    println!("Hello, world!");
}

fn is_palindrome(n: i32) -> bool {
    // Is a number the same forwards and backwards?

    let str = match n > 0 {
        true => n.to_string(),
        false => return false,
    };

    str.chars().rev().collect::<String>() == str
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
}
