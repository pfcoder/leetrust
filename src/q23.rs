use super::util::linked_list::{to_list, ListNode};
use std::slice;
pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = &mut dummy;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() {
            if l2.is_none() {
                head.as_mut().unwrap().next = l1;
                break;
            } else if l1.is_none() {
                head.as_mut().unwrap().next = l2;
                break;
            }

            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                // l1 < l2
                let tmp = l1.as_mut().unwrap().next.take();
                head.as_mut().unwrap().next = l1.take();
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                head.as_mut().unwrap().next = l2.take();
                l2 = tmp;
            }

            head = &mut head.as_mut().unwrap().next;
        }

        dummy.unwrap().next
    }
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn bin_merge(last_v: &mut [Option<Box<ListNode>>]) -> Vec<Option<Box<ListNode>>> {
            let mut tmp: Vec<Option<Box<ListNode>>> = Vec::new();
            let llen = last_v.len();
            for i in (0..llen).step_by(2) {
                if i < llen - 1 {
                    tmp.push(Solution::merge_two_lists(
                        last_v[i].take(),
                        last_v[i + 1].take(),
                    ));
                } else {
                    tmp.push(last_v[i].take());
                }
            }
            tmp
        };

        if lists.len() == 0 {
            return None;
        }

        let mut l = lists;
        let mut last_v = l.as_mut_slice();
        let mut tmp;
        loop {
            tmp = bin_merge(last_v);
            if tmp.len() == 1 {
                break;
            } else {
                unsafe {
                    last_v = slice::from_raw_parts_mut(tmp.as_mut_ptr(), tmp.len());
                }
            }
        }

        tmp.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}
