use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    fn find_bound(mut start: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, i32) {
        let mut step_count = 0;
        let mut move_p = start.take();
        while step_count < k - 1 {
            step_count += 1;
            if move_p.as_ref().unwrap().next == None {
                break;
            }
            move_p = move_p.unwrap().next.take();
        }

        (move_p, step_count)
    }

    /*fn swap(start: &mut Box<ListNode>, steps: i32) -> () {
        let mut step_count = 0;
        let mut move_p = start.as_ptr();
        while step_count < steps {
            step_count += 1;
            move_p = &mut move_p.next.as_mut().unwrap();
        }

        move_p.val = 1;
        start.val = 1;
    }*/

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head == None {
            return head;
        }

        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));

        //let (mut end, mut steps) = Solution::find_bound(dummy_head.as_mut().unwrap().next, k);
        //move_steps = steps;

        /*if move_steps == k - 1 {
            // swap
            for i in 0..k {
                //                let tmp2 = end.val;
                let tmp1 = move_start.unwrap().val;
                end.unwrap().val = tmp1;
                move_start.unwrap().val = 1;
            }
        }*/

        //println!("p:{} {}", move_start.val, end.val);
        //end.val = 1;
        //move_start.val = 1;

        //println!("p:{} {}", move_start.val, end.val);

        //loop {
        // advance pointer, and check if left items count is smaller than k
        //let mut move_p = &move_start;
        //println!("p v: {}", start_p.val);
        //println!("p:{} {}", move_start.val, move_p.val);

        // start switch
        /*if step_count == k - 1 {
            // has enough length
            for i in 0..k {
                let tmp1 = move_start.as_ref().unwrap().val;
                let tmp2 = move_p.val;
                move_p.val = tmp1;
            }
        }*/

        //}

        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 5),
            to_list(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 1),
            to_list(vec![1])
        );
    }
}
