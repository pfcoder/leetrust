pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut r: Vec<Vec<i32>> = vec![];
        if nums.len() < 3 {
            return r;
        }

        let mut s = Vec::from(nums);
        // sort
        s.sort();

        for i in 0..s.len() {
            if *s.get(i).unwrap() > 0 {
                return r;
            }

            if i > 0 && *s.get(i - 1).unwrap() == *s.get(i).unwrap() {
                continue;
            }

            let mut lindx = i + 1;
            let mut rindx = s.len() - 1;
            let s1 = *s.get(i).unwrap();
            while lindx < rindx {
                let s2 = *s.get(lindx).unwrap();
                let s3 = *s.get(rindx).unwrap();
                let sum = s1 + s2 + s3;
                if sum == 0 {
                    r.push(vec![s1, s2, s3]);
                    while lindx < rindx && *s.get(lindx).unwrap() == *s.get(lindx + 1).unwrap() {
                        lindx += 1;
                    }

                    while lindx < rindx && *s.get(rindx).unwrap() == *s.get(rindx - 1).unwrap() {
                        rindx -= 1;
                    }

                    lindx += 1;
                    rindx -= 1;
                } else if sum > 0 {
                    rindx -= 1;
                } else {
                    lindx += 1;
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
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![
                -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4,
                2, -5, -4, 4, 7
            ]),
            vec![
                vec![-10, 2, 8],
                vec![-10, 3, 7],
                vec![-10, 4, 6],
                vec![-10, 5, 5],
                vec![-9, 2, 7],
                vec![-9, 3, 6],
                vec![-9, 4, 5],
                vec![-8, 2, 6],
                vec![-8, 3, 5],
                vec![-8, 4, 4],
                vec![-7, -1, 8],
                vec![-7, 2, 5],
                vec![-7, 3, 4],
                vec![-6, -2, 8],
                vec![-6, -1, 7],
                vec![-6, 2, 4],
                vec![-5, -3, 8],
                vec![-5, -2, 7],
                vec![-5, -1, 6],
                vec![-5, 2, 3],
                vec![-4, -4, 8],
                vec![-4, -3, 7],
                vec![-4, -2, 6],
                vec![-4, -1, 5],
                vec![-3, -2, 5],
                vec![-3, -1, 4],
                vec![-2, -1, 3],
                vec![-1, -1, 2]
            ]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty_vec);
    }
}
