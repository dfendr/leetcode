fn main() {
    println!("Hello, world!");
}
#[derive(Debug)]
enum Numeral {
    I, // 1
    V, // 5
    X, // 10
    L, // 50
    C, // 100
    D, // 500 ?
    M, // 1000 ?
}

impl Numeral {}

fn roman_to_int(roman: &str) -> i32 {
    0
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int("III"), 3);
    assert_eq!(roman_to_int("IV"), 4);
    assert_eq!(roman_to_int("IX"), 9);
    assert_eq!(roman_to_int("LVIII"), 58);
    assert_eq!(roman_to_int("MCMXCIV"), 1994);
}
