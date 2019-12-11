pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut last_char: char = '0';
        s.as_bytes()
            .iter()
            .rev()
            .map(|b| {
                let r = match *b as char {
                    'I' => match last_char {
                        'V' | 'X' => -1,
                        _ => 1,
                    },
                    'V' => 5,
                    'X' => match last_char {
                        'L' | 'C' => -10,
                        _ => 10,
                    },
                    'L' => 50,
                    'C' => match last_char {
                        'D' | 'M' => -100,
                        _ => 100,
                    },
                    'D' => 500,
                    'M' => 1000,
                    _ => 0,
                };

                last_char = *b as char;
                r
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }
}
