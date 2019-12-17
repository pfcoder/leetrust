// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub struct Solution {}
use super::util::linked_list::{to_list, ListNode};
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        //let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        /*{
            let locate = || {
                // double pointer method
                let mut end = dummy_head.as_ref();

                // end start from n - 1 step
                let mut step = 0;
                while end != None && step < n {
                    end = end.unwrap().next.as_ref();
                    step += 1;
                }

                let mut start = &mut dummy_head;
                // now we have distance n, move pointers together until reach end
                while end != None {
                    end = end.unwrap().next.as_ref();
                    // start = &mut start.unwrap().next;
                }

                //start
            };

            let p = locate();
            {
                // start is point to ready to del before 1
                //let p = start.unwrap();
                // start.as_mut().unwrap().next = None;
                //let next = p.unwrap().next.as_ref().unwrap().next;
                //p.unwrap().next = next;
                //(*p).next = None;
            }
        }
        //start.as_mut().unwrap().next = start.as_ref().unwrap().next.as_ref().unwrap().next.take();
        dummy_head.unwrap().next*/

        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        {
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }
        let idx = len - n;
        {
            let mut p = dummy_head.as_mut();
            for _ in 0..(idx) {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
