use std::collections::HashMap;

struct Solution {}
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    let mut pos: usize = 0;
    let num_len: usize = nums.len();

    for val in &nums {
      hash.insert(target - val, pos);
      pos += 1;

      if pos == num_len {
        break;
      }

      let next = nums[pos];

      match hash.get(&next) {
        Some(pos_x) => return vec![*pos_x as i32, pos as i32],
        None => (),
      }
    }

    return vec![];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
  }
}
