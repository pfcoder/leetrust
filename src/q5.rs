use std::collections::HashMap;

pub struct Solution {}

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    let mut start = 0;
    let mut end = 0;
    let mut hash = HashMap::new();
    let mut pos = 0;

    for c in s.chars() {
      println!("process: {}", c);
      match hash.get(&c) {
        Some(v) => {
          println!("v: {} pos: {}", *v, pos);
          // found repeat, check
          let len = pos - *v + 1;
          let current = end - start + 1;
          if len > current {
            let half = len / 2;
            let mut is_break = false;
            for i in 0..half {
              let start_pos = *v + i as usize;
              let l = &s[start_pos..start_pos + 1];
              let r = &s[pos - i..pos - i + 1];

              println!("l: {} r: {}", l, r);
              if l != r {
                is_break = true;
                break;
              }
            }

            if !is_break {
              // found more longer, have a check on previous start
              if c == &s[end..end + 1].chars().get(0) {
                let prev = &s[start..start + 1];
                if (c == prev) {}
              }

              start = *v;
              end = pos;
            }
          }
        }
        None => (),
      }

      hash.insert(c, pos);
      pos += 1;
    }

    if end > start {
      s[start..end + 1].to_string()
    } else {
      String::from("")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_5() {
    //assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
    assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
    assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
    assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
    assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
    assert_eq!(Solution::longest_palindrome("".to_owned()), "");
  }
}
