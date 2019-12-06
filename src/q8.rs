pub struct Solution {}

impl Solution {
  pub fn my_atoi(str: String) -> i32 {
    let base: i64 = 2;
    let upper_bound: i64 = base.pow(31) - 1;
    let lower_bound: i64 = -base.pow(31);
    let mut result: i64 = 0;
    let mut is_num_start = false;
    let mut is_negtive = false;

    let process_end = |is_negtive: bool, result: i64| {
      if is_negtive {
        if -result <= lower_bound {
          return lower_bound as i32;
        }

        return -result as i32;
      }

      if result >= upper_bound {
        return upper_bound as i32;
      }

      return result as i32;
    };

    for c in str.chars() {
      match c {
        ' ' => {
          if is_num_start {
            return process_end(is_negtive, result);
          }
        } // ignore space
        '-' => {
          if is_num_start {
            return process_end(is_negtive, result);
          }
          is_negtive = true;
          is_num_start = true;
        }
        '+' => {
          if is_num_start {
            return process_end(is_negtive, result);
          }
          is_num_start = true;
        }
        '0'..='9' => {
          is_num_start = true;
          // process data add
          let mut safe_compute = result.checked_mul(10);
          if safe_compute == None {
            return process_end(is_negtive, upper_bound + 1);
          }

          safe_compute = safe_compute
            .unwrap()
            .checked_add(c.to_digit(10).unwrap() as i64);
          if safe_compute == None {
            return process_end(is_negtive, upper_bound + 1);
          }

          result = safe_compute.unwrap();
        }
        _ => {
          if !is_num_start {
            // number not start, but found other chars, abort
            // println!("found other chars before meet number");
            return 0;
          }

          // if num start, and meet others char, stop it
          return process_end(is_negtive, result);
        }
      }
    }

    return process_end(is_negtive, result);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_8() {
    assert_eq!(
      Solution::my_atoi("-9223372036854775809".to_string()),
      2147483647
    );
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
  }
}
