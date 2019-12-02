use std::collections::HashMap;
struct Solution {}

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut hash = HashMap::new();

    let seq: Vec<char> = s.chars().collect();
    let len = seq.len();

    let mut update_max = |start, end| {
      // println!("start end in closure : {} {}", start, end);
      // let start = move start;
      let seq_count = end - start + 1;
      if seq_count > max {
        max = seq_count;
      }

      //max
    };

    for c in seq {
      match hash.get(&c) {
        Some(v) => {
          // find repeat, but ignore if v over start
          if *v < start {
            // do it as fresh
            update_max(start, end);
          } else if (*v as usize) < len - 1 {
            start = *v + 1;
          }
        }
        None => {
          update_max(start, end);
        }
      }
      hash.insert(c, end);
      end += 1;

      println!("end: {}", end);
    }

    max
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_3() {
    assert_eq!(
      Solution::length_of_longest_substring("tmmzuxt".to_string()),
      5
    );
    assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
    assert_eq!(
      Solution::length_of_longest_substring("pwwkew".to_string()),
      3
    );
  }
}
