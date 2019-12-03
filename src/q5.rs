use std::collections::HashMap;

pub struct Solution {}

pub struct Handler {
  src: String,
  hash: HashMap<String, bool>,
}

impl Handler {
  pub fn new(src: String) -> Handler {
    Handler {
      src,
      hash: HashMap::new(),
    }
  }

  fn is_cycle(&mut self, i: usize, j: usize, b: &[u8]) -> bool {
    let key = [i.to_string(), j.to_string()].join("-");
    if b[i] != b[j] {
      self.hash.insert(key, false);
      return false;
    }

    if i == j {
      // ponit to same shoul not happen
      panic!("should not happend {} {}", i, j);
    }

    let dist = j - i;
    if dist == 1 || dist == 2 {
      // naboure or combine one
      self.hash.insert(key, true);
      return true;
    }

    match self.hash.get(&key) {
      Some(v) => {
        //println!("hash result:{}", *v);
        return *v;
      }
      None => {
        let result = self.is_cycle(i + 1, j - 1, b);
        //println!("hash insert for :{} {}", key, result);
        self.hash.insert(key, result);
        return result;
      }
    }

    // check if hit hash
    //println!("hash check for :{}", key);
  }

  pub fn check(&mut self) -> String {
    let mut start = 0;
    let mut end = 0;
    let mut pos = 0;
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
      for j in i + 1..len {
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
      //String::from("")
      return self.src[0..1].to_string();
    }
  }
}

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    let mut h = Handler::new(s);
    return h.check();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_5() {
    //assert_eq!(Solution::longest_palindrome("cyyoacmjwjubfkzrrbvquqkwhsxvmytmjvbborrtoiyotobzjmohpadfrvmxuagbdczsjuekjrmcwyaovpiogspbslcppxojgbfxhtsxmecgqjfuvahzpgprscjwwutwoiksegfreortttdotgxbfkisyakejihfjnrdngkwjxeituomuhmeiesctywhryqtjimwjadhhymydlsmcpycfdzrjhstxddvoqprrjufvihjcsoseltpyuaywgiocfodtylluuikkqkbrdxgjhrqiselmwnpdzdmpsvbfimnoulayqgdiavdgeiilayrafxlgxxtoqskmtixhbyjikfmsmxwribfzeffccczwdwukubopsoxliagenzwkbiveiajfirzvngverrbcwqmryvckvhpiioccmaqoxgmbwenyeyhzhliusupmrgmrcvwmdnniipvztmtklihobbekkgeopgwipihadswbqhzyxqsdgekazdtnamwzbitwfwezhhqznipalmomanbyezapgpxtjhudlcsfqondoiojkqadacnhcgwkhaxmttfebqelkjfigglxjfqegxpcawhpihrxydprdgavxjygfhgpcylpvsfcizkfbqzdnmxdgsjcekvrhesykldgptbeasktkasyuevtxrcrxmiylrlclocldmiwhuizhuaiophykxskufgjbmcmzpogpmyerzovzhqusxzrjcwgsdpcienkizutedcwrmowwolekockvyukyvmeidhjvbkoortjbemevrsquwnjoaikhbkycvvcscyamffbjyvkqkyeavtlkxyrrnsmqohyyqxzgtjdavgwpsgpjhqzttukynonbnnkuqfxgaatpilrrxhcqhfyyextrvqzktcrtrsbimuokxqtsbfkrgoiznhiysfhzspkpvrhtewthpbafmzgchqpgfsuiddjkhnwchpleibavgmuivfiorpteflholmnxdwewj".to_owned()), "aaaaa");
    assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
    assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
    assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
    assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
    assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
    assert_eq!(Solution::longest_palindrome("".to_owned()), "");
  }
}
