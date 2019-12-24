use super::util::linked_list::{to_list, ListNode};
pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    fn try_stack(start: Option<&Box<ListNode>>, k: i32) -> Vec<i32> {
        let mut step_count = 0;
        let mut move_p = start;
        let mut stack: Vec<i32> = Vec::new();
        while step_count < k {
            stack.push(move_p.as_ref().unwrap().val);
            step_count += 1;
            if move_p.as_ref().unwrap().next == None {
                break;
            }
            move_p = move_p.as_ref().unwrap().next.as_ref();
        }

        if step_count == k {
            stack
        } else {
            Vec::new()
        }
    }

    fn replace(start: Option<&Box<ListNode>>, mut stack: Vec<i32>) -> Option<&Box<ListNode>> {
        let mut end = start;
        while let Some(v) = stack.pop() {
            unsafe {
                let mut mx = Solution::im2mut(end.unwrap());
                mx.val = v;
            }

            end = end.unwrap().next.as_ref();
        }

        end
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

        let dummy_head = Some(Box::new(ListNode { val: 0, next: head }));

        let mut start = dummy_head.as_ref().unwrap().next.as_ref();

        while start != None {
            let stack = Solution::try_stack(start, k);
            if stack.len() == k as usize {
                start = Solution::replace(start, stack);
            } else {
                break;
            }
        }

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
