use std::time::{Duration, Instant};

pub struct Solution {}

pub struct Handler {
  src: String,
  map: [Option<bool>; 1000000],
}

impl Handler {
  pub fn new(src: String) -> Handler {
    Handler {
      src,
      map: [None; 1000000],
    }
  }

  fn is_cycle(&mut self, i: usize, j: usize, b: &[u8]) -> bool {
    let key = i * 1000 + j;
    if b[i] != b[j] {
      self.map[key] = Some(false);
      return false;
    }

    if i == j {
      // ponit to same shoul not happen
      panic!("should not happend {} {}", i, j);
    }

    let dist = j - i;
    if dist == 1 || dist == 2 {
      // naboure or combine one
      self.map[key] = Some(true);
      return true;
    }

    match self.map[key] {
      Some(v) => {
        return v;
      }
      None => {
        let result = self.is_cycle(i + 1, j - 1, b);
        self.map[key] = Some(result);
        return result;
      }
    }
  }

  pub fn check(&mut self) -> String {
    let mut start = 0;
    let mut end = 0;
    let len = self.src.len();

    if len == 0 {
      return String::from("");
    }

    if len == 1 {
      return self.src[0..1].to_string();
    }

    let new_str = String::from(self.src.to_string());
    let b = new_str.as_bytes();

    for i in 0..len {
      for j in (i..len).rev() {
        let current_len = j - i + 1;
        let current_max = end - start + 1;

        if current_len > current_max {
          let result = self.is_cycle(i, j, b);
          if result {
            // replace it
            start = i;
            end = j;
          }
        }
      }
    }

    if end > start {
      self.src[start..end + 1].to_string()
    } else {
      return self.src[0..1].to_string();
    }
  }
}

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    let start = Instant::now();
    let mut h = Handler::new(s);
    let r = h.check();

    let elapsed = start.elapsed();
    println!("execute spend: {:?}", elapsed);

    return r;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_5() {
    /*assert_eq!(
      Solution::longest_palindrome("zhqusxzrjcwgsdpcienkizutedcwrmowwolekockvyukyv".to_owned()),
      "aaaaa"
    );*/
    assert_eq!(Solution::longest_palindrome("cyyoacmjwjubfkzrrbvquqkwhsxvmytmjvbborrtoiyotobzjmohpadfrvmxuagbdczsjuekjrmcwyaovpiogspbslcppxojgbfxhtsxmecgqjfuvahzpgprscjwwutwoiksegfreortttdotgxbfkisyakejihfjnrdngkwjxeituomuhmeiesctywhryqtjimwjadhhymydlsmcpycfdzrjhstxddvoqprrjufvihjcsoseltpyuaywgiocfodtylluuikkqkbrdxgjhrqiselmwnpdzdmpsvbfimnoulayqgdiavdgeiilayrafxlgxxtoqskmtixhbyjikfmsmxwribfzeffccczwdwukubopsoxliagenzwkbiveiajfirzvngverrbcwqmryvckvhpiioccmaqoxgmbwenyeyhzhliusupmrgmrcvwmdnniipvztmtklihobbekkgeopgwipihadswbqhzyxqsdgekazdtnamwzbitwfwezhhqznipalmomanbyezapgpxtjhudlcsfqondoiojkqadacnhcgwkhaxmttfebqelkjfigglxjfqegxpcawhpihrxydprdgavxjygfhgpcylpvsfcizkfbqzdnmxdgsjcekvrhesykldgptbeasktkasyuevtxrcrxmiylrlclocldmiwhuizhuaiophykxskufgjbmcmzpogpmyerzovzhqusxzrjcwgsdpcienkizutedcwrmowwolekockvyukyvmeidhjvbkoortjbemevrsquwnjoaikhbkycvvcscyamffbjyvkqkyeavtlkxyrrnsmqohyyqxzgtjdavgwpsgpjhqzttukynonbnnkuqfxgaatpilrrxhcqhfyyextrvqzktcrtrsbimuokxqtsbfkrgoiznhiysfhzspkpvrhtewthpbafmzgchqpgfsuiddjkhnwchpleibavgmuivfiorpteflholmnxdwewj".to_owned()), "aaaaa");
    assert_eq!(Solution::longest_palindrome("bananas".to_owned()), "anana");
    assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
    assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
    assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
    assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
    assert_eq!(Solution::longest_palindrome("".to_owned()), "");
  }
}
