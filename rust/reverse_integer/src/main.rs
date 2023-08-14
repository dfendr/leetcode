fn main() {
    println!("Hello, world!");
}

fn reverse(n: i32) -> i32 {
    let mut negative = false;

    let str = match n > 0 {
        true => n.to_string(),
        false => {
            negative = true;
            (-n).to_string()
        }
    };

    let mut rev = str
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

    if negative {
        rev = -rev;
    }

    rev
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
}
