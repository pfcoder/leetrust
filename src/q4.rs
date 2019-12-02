struct Solution {}

pub struct Handler {
  nums1: Vec<i32>,
  nums2: Vec<i32>,
}

impl Handler {
  pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
    Handler { nums1, nums2 }
  }

  pub fn start(&self) -> f64 {
    self.bin_process(0, self.nums1.len())
  }

  pub fn bin_process(&self, start: usize, end: usize) -> f64 {
    let m = self.nums1.len();
    let n = self.nums2.len();
    // start with m/2
    let i = (end - start) / 2;
    let j = (m + n) / 2 - i;

    let (li, lj, ri, rj);

    if i > 0 {
      li = *self.nums1.get(i - 1).unwrap();
    } else {
      li = *self.nums1.get(i).unwrap();
    }

    if j > 0 {
      lj = *self.nums2.get(j - 1).unwrap();
    } else {
      lj = *self.nums2.get(j).unwrap();
    }
    ri = *self.nums1.get(i).unwrap();
    rj = *self.nums2.get(j).unwrap();

    println!(
      "seperate: i:{} j:{} start:{} end:{} li:{} lj:{} ri:{} rj:{}",
      i, j, start, end, li, lj, ri, rj
    );

    if li <= rj && lj <= ri {
      println!("found end point: {} {}", i, j);
      let lc: i32;
      let rc: i32;

      if li > lj {
        lc = li;
      } else {
        lc = lj;
      }

      if ri < rj {
        rc = ri
      } else {
        rc = rj
      }

      println!("lc:{} rc:{}", lc, rc);

      (lc as f64 + rc as f64) / 2.0
    } else {
      if li > rj {
        // left i too large, reduce i
        return self.bin_process(0, (end - start) / 2);
      } else if lj > ri {
        // left j too large, increate i
        return self.bin_process(0, end + (n - j) / 2);
      } else {
        println!("unexpected");
        0.0
      }
    }
  }
}

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let h = Handler::new(nums1, nums2);
    h.start()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // TODO: implementation
  #[test]
  fn test_4() {
    /*assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
      2.0
    );*/
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
      2.5
    );
  }
}
