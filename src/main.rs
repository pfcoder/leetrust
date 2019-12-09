use leetcode::q10::Solution;

fn main() {
  println!("start");
  println!(
    "result: {}",
    Solution::is_match("a".to_string(), ".*..a*".to_string())
  );
}
