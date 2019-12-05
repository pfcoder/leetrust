pub struct Solution {}

impl Solution {
  pub fn my_atoi(str: String) -> i32 {
    let base: i64 = 2;
    let upper_bound: i64 = base.pow(31) - 1;
    let lower_bound: i64 = -base.pow(31);
    let result: i64 = 0;

    for c in str.chars() {
      println!("process: {}", c);
    }

    result as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_8() {
    assert_eq!(Solution::my_atoi("aa".to_string()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
  }
}
