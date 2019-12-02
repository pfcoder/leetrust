use super::util::linked_list::{to_list, ListNode};

struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut l1_end = false;
    let mut l2_end = false;

    let mut h = Some(Box::new(ListNode::new(0)));
    let mut t = &mut h;
    let mut pos_inc = 0;

    loop {
      let lv = match l1 {
        Some(node) => {
          l1 = node.next;
          node.val
        }
        None => {
          l1_end = true;
          0
        }
      };

      let rv = match l2 {
        Some(node) => {
          l2 = node.next;
          node.val
        }
        None => {
          l2_end = true;
          0
        }
      };

      if l1_end && l2_end && pos_inc == 0 {
        break h.unwrap().next;
      }

      let mut pos_v = lv + rv + pos_inc;
      if pos_v >= 10 {
        pos_v = pos_v - 10;
        pos_inc = 1;
      } else {
        pos_inc = 0;
      }

      t.as_mut().unwrap().next = Some(Box::new(ListNode::new(pos_v)));
      t = &mut t.as_mut().unwrap().next;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_2() {
    assert_eq!(
      Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
      to_list(vec![7, 0, 8])
    );

    assert_eq!(
      Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
      to_list(vec![8, 9, 9, 9, 0, 0, 1])
    );

    assert_eq!(
      Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
      to_list(vec![0])
    )
  }
}
