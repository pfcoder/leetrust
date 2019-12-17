use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}
/*
struct Handler {
    merged: Option<Box<ListNode>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler { merged: None }
    }

    pub fn deep<'a>(
        &'a mut self,
        l1: Option<&'a mut Box<ListNode>>,
        l2: Option<&'a mut Box<ListNode>>,
    ) -> () /*Option<&'a Box<ListNode>>*/ {
        //let (mut l1, mut l2) = (l1, l2);
        if l1.is_none() {
            // self.merged = l2.and_then(|n| Some(*n));
            let v = l2.unwrap
            //return l2;
            return;
        }

        if l2.is_none() {
            self.merged = l1.and_then(|n| Some(*n));
            return;
        }

        if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
            //l1.as_mut().unwrap().next = self.deep(l1.as_ref().unwrap().next.as_ref(), l2);
            return;
        } else {
            //l2.as_mut().unwrap().next = self.deep(l1, l2.as_ref().unwrap().next.as_ref());
            return;
        }
    }
}*/
#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        /*let mut h = Handler::new();
        h.deep(l1.as_ref(), l2.as_ref());
        h.merged*/

        let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = &mut dummy;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() {
            if l2.is_none() {
                head.as_mut().unwrap().next = l1;
                println!("l2 none");
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
