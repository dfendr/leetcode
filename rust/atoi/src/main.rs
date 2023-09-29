fn main() {}

struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        let mut chars = s.chars();
        let mut result: i64 = 0;

        let sign = match chars.next() {
            Some('+') => 1,
            Some('-') => -1,
            Some(digit) if digit.is_ascii_digit() => {
                result = digit.to_digit(10).unwrap() as i64;
                1
            }
            _ => 0,
        };

        for c in chars {
            if !c.is_ascii_digit() {
                break;
            }
            result = result
                .saturating_mul(10)
                .saturating_add(c.to_digit(10).unwrap() as i64);

            // handle over/under flow
            if result.saturating_mul(sign) > i32::MAX as i64 {
                return i32::MAX;
            }
            if result.saturating_mul(sign) < i32::MIN as i64 {
                return i32::MIN;
            }
        }
        (result * sign) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_negative_integer() {
        assert_eq!(Solution::my_atoi("-42".to_string()), -42);
    }

    #[test]
    fn test_leading_whitespace() {
        assert_eq!(Solution::my_atoi("    42".to_string()), 42);
    }

    #[test]
    fn test_whitespace_and_sign() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn test_plus_sign() {
        assert_eq!(Solution::my_atoi("+42".to_string()), 42);
    }

    #[test]
    fn test_non_integer_characters() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn test_non_integer_characters_after() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn test_overflow() {
        assert_eq!(Solution::my_atoi("91283472332".to_string()), i32::MAX);
    }

    #[test]
    fn test_underflow() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::my_atoi("".to_string()), 0);
    }

    #[test]
    fn test_only_whitespace() {
        assert_eq!(Solution::my_atoi("   ".to_string()), 0);
    }

    #[test]
    fn test_only_sign() {
        assert_eq!(Solution::my_atoi("-".to_string()), 0);
    }

    #[test]
    fn test_decimal_number() {
        assert_eq!(Solution::my_atoi("42.73".to_string()), 42);
    }

    #[test]
    fn test_decimal_number_negative() {
        assert_eq!(Solution::my_atoi("-42.73".to_string()), -42);
    }

    #[test]
    fn test_decimal_number_whitespace() {
        assert_eq!(Solution::my_atoi("  42.73".to_string()), 42);
    }

    #[test]
    fn test_multiple_signs() {
        assert_eq!(Solution::my_atoi("+-42".to_string()), 0);
    }

    #[test]
    fn test_multiple_signs_reversed() {
        assert_eq!(Solution::my_atoi("-+42".to_string()), 0);
    }

    #[test]
    fn test_non_digit_after_sign() {
        assert_eq!(Solution::my_atoi("+a42".to_string()), 0);
    }

    #[test]
    fn test_non_digit_after_whitespace_and_sign() {
        assert_eq!(Solution::my_atoi("  +a42".to_string()), 0);
    }

    #[test]
    fn test_sign_at_the_end() {
        assert_eq!(Solution::my_atoi("42-".to_string()), 42);
    }

    #[test]
    fn test_max_int() {
        assert_eq!(Solution::my_atoi(i32::MAX.to_string()), i32::MAX);
    }

    #[test]
    fn test_min_int() {
        assert_eq!(Solution::my_atoi(i32::MIN.to_string()), i32::MIN);
    }

    #[test]
    fn test_just_over_max_int() {
        assert_eq!(Solution::my_atoi("2147483648".to_string()), i32::MAX);
    }

    #[test]
    fn test_just_under_min_int() {
        assert_eq!(Solution::my_atoi("-2147483649".to_string()), i32::MIN);
    }

    #[test]
    fn test_only_decimal_point() {
        assert_eq!(Solution::my_atoi(".".to_string()), 0);
    }

    #[test]
    fn test_decimal_without_leading_digit() {
        assert_eq!(Solution::my_atoi(".42".to_string()), 0);
    }

    #[test]
    fn test_large_string() {
        let large_str = "1".repeat(1_000_000);
        assert_eq!(Solution::my_atoi(large_str), i32::MAX);
    }

    #[test]
    fn test_mixed_whitespace_and_signs() {
        assert_eq!(Solution::my_atoi("  -  4 2".to_string()), 0);
    }

    #[test]
    fn test_multiple_decimal_points() {
        assert_eq!(Solution::my_atoi("42.4.2".to_string()), 42);
    }

    #[test]
    fn test_sign_and_decimal_mixed() {
        assert_eq!(Solution::my_atoi("-.42".to_string()), 0);
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(Solution::my_atoi("@42".to_string()), 0);
    }

    #[test]
    fn test_alpha_numeric_mixed() {
        assert_eq!(Solution::my_atoi("4a2".to_string()), 4);
    }

    #[test]
    fn test_whitespace_at_end() {
        assert_eq!(Solution::my_atoi("42 ".to_string()), 42);
    }

    #[test]
    fn test_whitespace_then_sign_at_end() {
        assert_eq!(Solution::my_atoi("42 -".to_string()), 42);
    }
}
