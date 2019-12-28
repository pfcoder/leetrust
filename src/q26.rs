pub struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let input_len = nums.len();
        if input_len <= 1 {
            return input_len as i32;
        }

        let mut j = 0;

        for i in 1..input_len {
            let tmp = *nums.get(i).unwrap();
            if tmp == *nums.get(j).unwrap() {
                continue;
            }
            if i > j {
                nums[j + 1] = tmp;
                j += 1;
            }
        }

        unsafe {
            nums.set_len(j + 1);
        }

        (j + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
        let mut vec1 = vec![1, 1, 1, 1, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 3]);
        let mut vec2 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec2), 2);
        assert_eq!(vec2, vec![1, 2]);
    }
}
