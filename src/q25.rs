use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head == None {
            return head;
        }

        let mut mut_head = head;
        let mut p = mut_head.as_mut();

        //loop {
        // advance pointer, and check if left items count is smaller than k
        let mut step_count = 0;
        let mut move_p = p.as_mut().unwrap().as_mut();
        //println!("p v: {}", start_p.val);
        while step_count < k {
            step_count += 1;
            move_p = move_p.next.as_mut().unwrap().as_mut();
        }

        println!("p:{}", move_p.val);

        //}

        mut_head
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
