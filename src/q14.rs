pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut r = String::from("");
        let mut i = 0;

        if strs.len() == 0 {
            return r;
        }

        loop {
            let mut pos_byte: u8 = 0;

            for s in &strs {
                if i >= s.len() {
                    return r;
                }

                let pc = s.as_bytes()[i];

                if pos_byte == 0 {
                    pos_byte = pc;
                } else if pc != pos_byte {
                    return r;
                }
            }

            // pass check
            r.push_str(&(pos_byte as char).to_string());
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}
