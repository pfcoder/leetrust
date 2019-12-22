use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return head;
        }
        let mut mut_head = head;
        let mut p = mut_head.as_mut().unwrap();

        while p.next != None {
            let tmp1 = p.val;
            let tmp2 = p.next.as_mut().unwrap().val;

            p.val = tmp2;
            p.next.as_mut().unwrap().val = tmp1;

            p = p.next.as_mut().unwrap();
            if p.next == None {
                break;
            } else {
                p = p.next.as_mut().unwrap();
            }
        }

        mut_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}
