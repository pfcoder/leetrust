pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut r = 0;

        if nums.len() < 3 {
            return r;
        }

        let mut s = Vec::from(nums);
        s.sort();
        let mut min_diff: i32 = std::i32::MAX;

        for i in 0..s.len() {
            let mut lidx = i + 1;
            let mut ridx = s.len() - 1;

            let s1 = s.get(i).unwrap();
            while lidx < ridx {
                let sum = s1 + s.get(lidx).unwrap() + s.get(ridx).unwrap();
                let mut diff = sum - target;
                if diff < 0 {
                    diff = -diff;
                }

                if diff <= min_diff {
                    r = sum;
                    min_diff = diff;
                }

                println!("xxxx: {} {} {} {} {}", lidx, ridx, sum, diff, min_diff);

                if sum > target {
                    ridx -= 1;
                } else if sum < target {
                    lidx += 1;
                } else {
                    return r;
                }
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 5, 10, 11], 12), 13);
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}
