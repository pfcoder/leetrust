use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    fn find_bound(start: Option<&Box<ListNode>>, k: i32) -> (Option<&Box<ListNode>>, i32) {
        let mut step_count = 0;
        let mut move_p = start;
        while step_count < k - 1 {
            step_count += 1;
            if move_p.as_ref().unwrap().next == None {
                break;
            }
            move_p = move_p.as_ref().unwrap().next.as_ref();
        }

        (move_p, step_count)
    }

    fn swap(start: Option<&Box<ListNode>>, end: Option<&Box<ListNode>>, steps: i32) -> () {
        let mut step_count = 0;
        let mut move_start = start.unwrap();
        let mut move_end = end.unwrap();
        while step_count < steps {
            step_count += 1;
            unsafe {
                let tmp1 = move_start.val;
                let tmp2 = move_end.val;
                Solution::im2mut(move_start).val = tmp2;
                Solution::im2mut(move_end).val = tmp1;
            }
        }
    }

    unsafe fn im2mut<T>(reference: &T) -> &mut T {
        let const_ptr = reference as *const T;
        let mut_ptr = const_ptr as *mut T;
        &mut *mut_ptr
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head == None {
            return head;
        }

        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));

        let mut start = dummy_head.as_ref().unwrap().next.as_ref();

        let (mut end, mut steps) = Solution::find_bound(start, k);

        let tmp1 = end.unwrap();
        let tmp2 = start.unwrap();

        unsafe {
            let mut tmp = Solution::im2mut(tmp1);
            tmp.val = 1;
        }

        //let x = Box::into_raw(*end.unwrap());

        //end.unwrap().val = 1;
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
