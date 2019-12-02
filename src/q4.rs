pub struct Solution {}

pub struct Handler {
  nums1: Vec<i32>,
  nums2: Vec<i32>,
}

impl Handler {
  pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
    Handler { nums1, nums2 }
  }

  pub fn start(&self) -> f64 {
    // check input
    let len1 = self.nums1.len();
    let len2 = self.nums2.len();
    if len1 == 0 {
      // return half nums2
      if len2 % 2 == 0 {
        let idx = len2 / 2;
        return ((self.nums2[idx] + self.nums2[idx - 1]) as f64) / 2.0;
      } else {
        return self.nums2[len2 / 2] as f64;
      }
    }

    if len2 == 0 {
      // return half nums2
      if len1 % 2 == 0 {
        let idx = len1 / 2;
        return ((self.nums1[idx] + self.nums1[idx - 1]) as f64) / 2.0;
      } else {
        return self.nums1[len1 / 2] as f64;
      }
    }

    let init = self.nums1.len() as i32 / 2;
    let idx = if init >= 1 { init - 1 } else { 0 };
    self.bin_process(idx)
  }

  pub fn bin_process(&self, li_idx: i32) -> f64 {
    let m = self.nums1.len();
    let n = self.nums2.len();
    let mut lj_idx: i32 = -1;
    let mut li_len: usize = 0;
    let mut lj_len: usize = 0;

    if li_idx >= 0 {
      li_len = (li_idx + 1) as usize;
    }

    let tmp = (m + n) as f32 / 2.0 - li_len as f32;
    if tmp < 0.0 {
      lj_len = 0;
    } else {
      lj_len = tmp as usize;
    }
    println!("tmp: {} lj_len: {} m:{} n: {}", tmp, lj_len, m, n);

    if lj_len > 0 {
      lj_idx = (lj_len - 1) as i32;
    } else {
      //lj_idx = 0;
      //lj_len = 0;
    }

    let li: Option<i32>;
    let lj: Option<i32>;
    let ri: Option<i32>;
    let rj: Option<i32>;

    li = if li_idx >= 0 {
      Some(*self.nums1.get(li_idx as usize).unwrap())
    } else {
      None
    };

    ri = if li_idx < (m - 1) as i32 {
      Some(*self.nums1.get((li_idx + 1) as usize).unwrap())
    } else {
      None
    };

    lj = if lj_idx >= 0 {
      Some(*self.nums2.get(lj_idx as usize).unwrap())
    } else {
      None
    };

    rj = if lj_idx < (n - 1) as i32 {
      Some(*self.nums2.get((lj_idx + 1) as usize).unwrap())
    } else {
      None
    };

    println!(
      "seperate: left i index:{} left j index:{} li:{:?} lj:{:?} ri:{:?} rj:{:?}",
      li_idx, lj_idx, li, lj, ri, rj
    );

    if li != None && rj != None && li.unwrap() > rj.unwrap() {
      // left i part too large
      let new_idx;
      if li_idx == 0 {
        new_idx = -1;
      } else {
        let tmp = (li_idx + 1) % 2;
        if tmp == 0 {
          new_idx = (li_idx + 1) / 2 - 1;
        } else {
          new_idx = (li_idx + 1) / 2;
        }
      };

      //return self.bin_process(new_idx);
      return 0.0;
    }

    if lj != None && ri != None && lj.unwrap() > ri.unwrap() {
      // left j part too large
      if li_idx != (m - 1) as i32 {
        let mut new_idx = (m as i32 - li_idx - 1) / 2;
        if new_idx == 0 {
          new_idx = 1;
        }
        //return self.bin_process(li_idx + new_idx);
        return 0.0;
      } else {
        // left i reach end, end all
      }
    }

    let lc;
    let rc;
    if li == None {
      lc = lj.unwrap();
    } else if lj == None {
      lc = li.unwrap();
    } else {
      lc = if li.unwrap() > lj.unwrap() {
        li.unwrap()
      } else {
        lj.unwrap()
      }
    }

    if ri == None {
      rc = rj.unwrap();
    } else if rj == None {
      rc = ri.unwrap();
    } else {
      rc = if ri.unwrap() < rj.unwrap() {
        ri.unwrap()
      } else {
        rj.unwrap()
      }
    }

    println!("lc: {}  rc: {}", lc, rc);

    if (m + n) % 2 == 0 {
      (lc as f64 + rc as f64) / 2.0
    } else {
      if ((li_len + lj_len) as f32) < (((m + n) as f32) / 2.0) {
        rc as f64
      } else {
        lc as f64
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
    //Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 6, 7, 8], vec![5]);
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 6, 7, 8], vec![5]),
      4.5
    );
    /* assert_eq!(
      Solution::find_median_sorted_arrays(vec![2, 3, 4, 5, 6, 7], vec![1]),
      4.0
    );
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
      2.0
    );
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 2,], vec![3, 4]),
      2.5
    );
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 2, 5], vec![3, 4]),
      3.0
    );*/
  }
}
