use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}

struct Handler {
    merged: Option<Box<ListNode>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler { merged: None }
    }

    pub fn deep<'a>(
        &'a mut self,
        l1: Option<&'a Box<ListNode>>,
        l2: Option<&'a Box<ListNode>>,
    ) -> Option<&'a Box<ListNode>> {
        //let (mut l1, mut l2) = (l1, l2);
        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
            //l1.as_mut().unwrap().next = self.deep(l1.as_ref().unwrap().next.as_ref(), l2);
            return l1;
        } else {
            //l2.as_mut().unwrap().next = self.deep(l1, l2.as_ref().unwrap().next.as_ref());
            return l2;
        }
    }
}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut h = Handler::new();
        h.deep(l1.as_ref(), l2.as_ref());
        h.merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
