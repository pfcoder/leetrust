pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut r: Vec<String> = Vec::new();
        Solution::process(&mut r, 0, 0, &mut Vec::new(), n);
        r
    }

    fn process(res: &mut Vec<String>, l: i32, r: i32, tmp: &mut Vec<char>, n: i32) {
        if l == n && r == n {
            // reach balance
            res.push(tmp.iter().collect());
        } else {
            if l < n {
                tmp.push('(');
                Solution::process(res, l + 1, r, tmp, n);
            }

            if r < l {
                tmp.push(')');
                Solution::process(res, l, r + 1, tmp, n);
            }
        }

        tmp.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
