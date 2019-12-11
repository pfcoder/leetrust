pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut max: i32 = 0;

        while l < r {
            let area: i32;
            let lh: i32 = *height.get(l).unwrap();
            let rh: i32 = *height.get(r).unwrap();
            let w: i32 = (r - l) as i32;
            if lh < rh {
                area = lh * w;
                l += 1;
            } else {
                area = rh * w;
                r -= 1;
            }

            if area > max {
                println!("max: {} {} {}", lh, rh, area);
                max = area;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![6, 9]), 6);
        assert_eq!(Solution::max_area(vec![1, 1, 2, 1, 1, 1]), 5);
    }
}
