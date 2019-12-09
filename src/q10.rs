pub struct Solution {}

impl Solution {
  pub fn is_match(s: String, p: String) -> bool {
    return Handler::new(&s[..], &p[..]).start();
  }
}

struct Handler<'a> {
  s: &'a [u8],
  p: &'a [u8],
  dp: Vec<Vec<bool>>,
}

impl Handler<'_> {
  pub fn new<'a>(s: &'a str, p: &'a str) -> Handler<'a> {
    Handler {
      s: s.as_bytes(),
      p: p.as_bytes(),
      dp: vec![vec![false; s.len() + 1]; p.len() + 1],
    }
  }

  pub fn start(&mut self) -> bool {
    if self.p.len() == 0 && self.s.len() == 0 {
      return true;
    }

    if self.p.len() == 0 {
      return false;
    }
    return self.match_check(0, 0);
  }

  fn match_check(&mut self, sidx: usize, pidx: usize) -> bool {
    let slen = self.s.len();
    let plen = self.p.len();

    println!(
      "enter: sidx:{} pidx:{} slen:{} plen:{}",
      sidx, pidx, slen, plen
    );

    let i = sidx;
    let j = pidx;

    //loop {
    if i >= slen && j >= plen {
      return true;
    }

    println!("process i:{} j:{}", i, j);
    // look forward pidx 2 for check if it's star match
    if j < plen - 1 && self.p[j + 1] as char == '*' {
      // try 0 match or dynamic match
      // ignore match
      if !self.match_check(i, j + 2) {
        // dynamic star match
        println!("r p i:{} j:{}", i, j);
        if i < slen && (self.p[j] as char == '.' || self.p[j] as char == self.s[i] as char) {
          return self.match_check(i + 1, j);
        } else {
          return false;
        }
      } else {
        return true;
      }
    } else {
      // direct compare
      if slen > 0
        && i <= slen - 1
        && j <= plen - 1
        && (self.p[j] as char == '.' || self.p[j] as char == self.s[i] as char)
      {
        println!("normal match i:{} j:{}", i, j);
        return self.match_check(i + 1, j + 1);
      } else {
        println!("i:{} j: {} false 2", i, j);
        return false;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_10() {
    assert_eq!(
      Solution::is_match(
        "aaaaaaaaaaaaab".to_string(),
        "a*a*a*a*a*a*a*a*a*a*c".to_string()
      ),
      false
    );
    assert_eq!(Solution::is_match("a".to_string(), "".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), ".".to_string()), false);
    assert_eq!(
      Solution::is_match("a".to_string(), ".*..a*".to_string()),
      false
    );
    assert_eq!(
      Solution::is_match("ab".to_string(), ".*..".to_string()),
      true
    );
    assert_eq!(
      Solution::is_match("a".to_string(), "ab*a".to_string()),
      false
    );
    assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
    assert_eq!(
      Solution::is_match("ab".to_string(), ".*c".to_string()),
      false
    );
    assert_eq!(
      Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()),
      true
    );
    assert_eq!(
      Solution::is_match("aaa".to_string(), "aaaa".to_string()),
      false
    );
    assert_eq!(
      Solution::is_match("aaa".to_string(), "a*a".to_string()),
      true
    );
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(
      Solution::is_match("aaaaaaa".to_string(), "a*".to_string()),
      true
    );
    assert_eq!(
      Solution::is_match("aaaaaaab".to_string(), "a*".to_string()),
      false
    );
    assert_eq!(
      Solution::is_match("baaaaaaab".to_string(), "ba*b".to_string()),
      true
    );
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    assert_eq!(
      Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
      false
    );
  }
}
