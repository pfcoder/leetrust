pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        Handler::new(digits).start()
    }
}

struct Handler {
    s: String,
    n2a: [String; 8],
    r: Vec<String>,
    inter: String,
}

impl Handler {
    pub fn new(s: String) -> Handler {
        Handler {
            s,
            n2a: [
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jkl".to_string(),
                "mno".to_string(),
                "pqrs".to_string(),
                "tuv".to_string(),
                "wxyz".to_string(),
            ],
            r: Vec::new(),
            inter: "".to_string(),
        }
    }
    pub fn start(&mut self) -> Vec<String> {
        self.process(0, 0);
        self.r.clone()
    }

    fn process(&mut self, row: usize, column: usize) -> () {
        let nidx = (self.s.as_bytes()[row] as char)
            .to_string()
            .parse::<usize>()
            .unwrap();
        let row_str = &self.n2a[nidx - 2];

        if column >= row_str.len() {
            return;
        } else {
            self.inter.push(row_str.as_bytes()[column] as char);
        }

        if row >= self.s.len() - 1 {
            self.r.push(self.inter.clone());
        } else {
            self.process(row + 1, 0);
        }
        self.inter.pop();
        self.process(row, column + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
