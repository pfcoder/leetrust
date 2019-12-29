use std::collections::HashMap;
use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let str_len = haystack.len();
        let pat_len = needle.len();
        if str_len == 0 && pat_len != 0 {
            return -1;
        }

        if pat_len == 0 {
            return 0;
        }

        let dp = Solution::setup_kmp(&needle);

        // start search
        let mut j = 0;
        for i in 0..str_len {
            let p = haystack.as_bytes()[i];
            j = *dp[j].get(&p).unwrap_or(&0);

            //println!("p:{}, j:{}", p, j);

            if j == pat_len {
                return i as i32 - pat_len as i32 + 1;
            }
        }

        -1
    }

    fn setup_kmp(pat: &str) -> Vec<HashMap<u8, usize>> {
        let mut char_set: HashSet<u8> = HashSet::new();
        let pat_bytes = pat.as_bytes();

        let pat_len = pat.len();
        let mut dp: Vec<HashMap<u8, usize>> = vec![HashMap::new(); pat_len];

        dp[0].insert(pat_bytes[0], 1);
        let mut prev_state = 0;
        char_set.insert(pat_bytes[0]);

        for i in 1..pat_len {
            let c = pat_bytes[i];
            char_set.insert(c);

            for pc in &char_set {
                if c == *pc {
                    // move state on
                    dp[i].insert(c, i + 1);
                } else {
                    let pstate = *dp[prev_state].get(pc).unwrap_or(&0);
                    dp[i].insert(*pc, pstate);
                }
            }

            prev_state = *dp[prev_state].get(&c).unwrap_or(&0);
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(
            Solution::str_str("mississippi".to_string(), "pi".to_string()),
            9
        )
    }
}
