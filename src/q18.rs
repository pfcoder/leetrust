pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut r: Vec<Vec<i32>> = vec![];
        if nums.len() < 4 {
            return r;
        }

        let mut s = Vec::from(nums);
        // sort
        s.sort();

        for i in 0..s.len() {
            if i > 0 && *s.get(i - 1).unwrap() == *s.get(i).unwrap() {
                continue;
            }
            let s1 = *s.get(i).unwrap();
            for j in i + 1..s.len() {
                if j > i + 1 && *s.get(j - 1).unwrap() == *s.get(j).unwrap() {
                    continue;
                }

                let mut lindx = j + 1;
                let mut rindx = s.len() - 1;
                let s2 = *s.get(j).unwrap();
                while lindx < rindx {
                    let s3 = *s.get(lindx).unwrap();
                    let s4 = *s.get(rindx).unwrap();
                    println!("s1 {} s2 {} s3 {} s4 {}", s1, s2, s3, s4);
                    let sum = s1 + s2 + s3 + s4;
                    if sum == target {
                        r.push(vec![s1, s2, s3, s4]);
                        while lindx < rindx && *s.get(lindx).unwrap() == *s.get(lindx + 1).unwrap()
                        {
                            lindx += 1;
                        }

                        while lindx < rindx && *s.get(rindx).unwrap() == *s.get(rindx - 1).unwrap()
                        {
                            rindx -= 1;
                        }

                        lindx += 1;
                        rindx -= 1;
                    } else if sum > target {
                        rindx -= 1;
                    } else {
                        lindx += 1;
                    }
                }
            }
        }
        println!("r len:{}", r.len());

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: build a macro for arbitrary match
    #[test]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![0, 0, 0, 0], 0),
            vec![vec![0, 0, 0, 0]]
        );
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-1, 0, 0, 1], vec![-2, 0, 0, 2], vec![-2, -1, 1, 2]]
        );
    }
}
