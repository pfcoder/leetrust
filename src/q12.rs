pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let process_unit = |x: i32| {
            let s4: char;
            let u4: char;
            let u9: char;
            let unit: i32;

            if x < 10 {
                s4 = 'I';
                u4 = 'V';
                u9 = 'X';
                unit = x;
            } else if x < 100 {
                s4 = 'X';
                u4 = 'L';
                u9 = 'C';
                unit = x / 10;
            } else if x < 1000 {
                s4 = 'C';
                u4 = 'D';
                u9 = 'M';
                unit = x / 100;
            } else {
                s4 = 'M';
                u4 = '?';
                u9 = '?';
                unit = x / 1000;
            }

            match unit {
                1..=3 => (0..unit).map(|_| s4.to_string()).collect::<String>(),
                4 => format!("{}{}", s4, u4),
                5..=8 => format!(
                    "{}{}",
                    u4.to_string(),
                    (0..unit - 5).map(|_| s4.to_string()).collect::<String>()
                ),
                9 => format!("{}{}", s4, u9),
                _ => "".to_string(),
            }
        };

        let mut vec: Vec<String> = Vec::new();
        let mut t = num;
        let mut count = 0;
        let base = 10;
        loop {
            if t == 0 {
                return vec.iter().rev().map(|s| s.to_string()).collect::<String>();
            }

            let m = t % base * base.pow(count);
            let r = process_unit(m);
            println!("process: {} {} {}", m, count, r);
            vec.push(r);
            count += 1;

            t = t / 10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
