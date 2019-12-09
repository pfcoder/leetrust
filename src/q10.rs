pub struct Solution {}

impl Solution {
  /*
  pub fn is_match(s: String, p: String) -> bool {
    let s_len = s.len();
    let p_len = p.len();

    println!("process: {} {} {} {}", s, p, s_len, p_len);

    if s_len == 0 || p_len == 0 {
      return false;
    }

    let mut s_pos = 0;
    let mut p_pos = 0;
    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();

    let mut is_star_match_start = false;
    let mut star_match_target: char = '.';
    let mut last_star_matched = false;
    let mut last_conusmed = false;

    loop {
      let sc = s_bytes[s_pos];
      let pc = p_bytes[p_pos];
      let mut s_moveon = false;
      let mut p_moveon = false;

      println!(
        "enter p:{} s:{} m:{} l:{}",
        p_pos, s_pos, is_star_match_start, last_star_matched
      );

      if is_star_match_start {
        // in * match, p previous char is target
        if sc as char == star_match_target || star_match_target == '.' {
          // continue, s move on, p keep
          s_moveon = true;
          last_star_matched = true;
        } else {
          // s move on to new stage, * match stop, ignore s step, p continue
          is_star_match_start = false;
          if p_pos >= p_len - 1 {
            // p reach end, but s still has non-match
            println!(
              "p:{} s:{} plen:{} slen:{} false aa p reach end",
              p_pos, s_pos, p_len, s_len
            );
            return false;
          }
          p_moveon = true;
        }
      } else {
        if p_pos < p_len - 1 && p_bytes[p_pos + 1] as char == '*' {
          is_star_match_start = true;
          star_match_target = pc as char;
          p_pos += 1;
        } else {
          if sc == pc || pc as char == '.' {
            if last_conusmed {
              if last_star_matched {
              } else if pc as char != '.' {
                return false;
              }
            }
            s_moveon = true;
            p_moveon = true;
            last_star_matched = false;
          } else {
            return false;
          }
        }
      }

      println!(
        "p:{} s:{} pm:{} sm:{} match_start:{}",
        p_pos, s_pos, p_moveon, s_moveon, is_star_match_start
      );

      if s_pos == s_len - 1 && p_pos == p_len - 1 {
        // reach end
        return true;
      }

      if s_moveon {
        if s_pos < s_len - 1 {
          s_pos += 1;
        } else {
          last_conusmed = true;
          // s reach end but p not, check if next p is star match
          if p_pos < p_len - 2 && p_bytes[p_pos + 2] as char == '*' {
            is_star_match_start = false;
            p_pos += 1;
            continue;
          } else if is_star_match_start {
            // in star match, move on p
            is_star_match_start = false;
            p_pos += 1;
            continue;
          }
          /*else if p_bytes[p_pos + 1] as char == '.' {
            is_star_match_start = false;
            p_pos += 1;
            continue;
          }*/
          else {
            // false;
            return false;
          }
        }
      }
      if p_moveon {
        if p_pos < p_len - 1 {
          p_pos += 1;
        } else if !is_star_match_start {
          return false;
        }
      }
    }
  }*/

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
