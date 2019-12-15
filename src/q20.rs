pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut pool: Vec<char> = Vec::new();

        let pop_check = |c| match c {
            '[' => ']',
            '{' => '}',
            '(' => ')',
            _ => 0 as char,
        };

        for c in s.chars() {
            match c {
                '[' | '{' | '(' => pool.push(c),
                ']' | '}' | ')' => {
                    if pool.len() == 0 || pop_check(pool.pop().unwrap()) != c {
                        return false;
                    }
                }
                _ => (),
            }
        }

        return pool.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("()[{}".to_string()), false);
    }
}
