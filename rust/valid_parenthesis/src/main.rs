fn main() {
    println!("Hello, world!");
    println!("{}", is_valid("()[]{}"));
    println!("{}", is_valid("(("));
    println!("{}", is_valid(")}"));
}

fn is_valid(s: &str) -> bool {
    // Early Exit Condition
    // If starting with closed paraenthesis or ending with open parenthesis
    if ")}]".contains(s.chars().next().unwrap()) || "{[(".contains(s.chars().last().unwrap()) {
        return false;
    }

    let mut paren_stack: Vec<char> = Vec::new();
    for char in s.chars() {
        match "([{".contains(char) {
            true => paren_stack.push(char),
            false => match (paren_stack.pop(), char) {
                (Some('('), ')') => continue,
                (Some('['), ']') => continue,
                (Some('{'), '}') => continue,
                _ => return false, // is only false if invalid
            },
        };
    }
    if !paren_stack.is_empty() {
        return false;
    }
    true // must be valid if it reaches this point
}

#[test]
fn test_is_valid() {
    assert_eq!(is_valid("()"), true);
    assert_eq!(is_valid("[])"), false);
    assert_eq!(is_valid("(){}}{"), false);
    assert_eq!(is_valid(")}"), false);
    assert_eq!(is_valid("(("), false);
    assert_eq!(is_valid("()[]{}"), true);
    assert_eq!(is_valid("(]"), false);
    assert_eq!(is_valid("([)]"), false);
    assert_eq!(is_valid("{[]}"), true);
}
