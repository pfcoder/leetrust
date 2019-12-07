pub struct Solution {}

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      return false;
    }
    let mut reverse: i32 = 0;
    let mut tmp = x;
    loop {
      if tmp == 0 {
        // println!("result: {}", reverse);
        return reverse == x;
      }

      reverse = reverse * 10 + tmp % 10;
      tmp = tmp / 10;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_9() {
    assert_eq!(Solution::is_palindrome(-32), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(0), true);
    assert_eq!(Solution::is_palindrome(9), true);
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(2222), true);
    assert_eq!(Solution::is_palindrome(11222211), true);
  }
}
