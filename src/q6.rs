use std::str;
pub struct Solution {}

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    let str_len = s.len();

    if str_len <= num_rows as usize || num_rows <= 1 {
      return s;
    }

    let seg_size = num_rows * 2 - 2;
    let mut seg_count = str_len as i32 / seg_size;

    if str_len as i32 % seg_size > 0 {
      seg_count += 1;
    }

    let mut result = String::new();
    let bytes = s.as_bytes();

    for i in 0..num_rows {
      //println!("processing row: {}", i);
      for j in 0..seg_count {
        let start = i + j as i32 * seg_size;
        let mut end = start as i32 + seg_size - 2 * i;

        if start >= str_len as i32 {
          break;
        }

        if i == 0 || end < 0 || end < start || end >= str_len as i32 {
          end = start;
        }

        result.push_str(str::from_utf8(&bytes[start as usize..start as usize + 1]).unwrap());
        if end != start {
          result.push_str(str::from_utf8(&bytes[end as usize..end as usize + 1]).unwrap());
        }

        //println!("seg str: {} {} {}", result, start, end);
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_6() {
    assert_eq!(
      Solution::convert("LEETCODEISHIRING".to_string(), 3),
      "LCIRETOESIIGEDHN"
    );
    assert_eq!(
      Solution::convert("PAYPALISHIRING".to_string(), 3),
      "PAHNAPLSIIGYIR"
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
    assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
  }
}
