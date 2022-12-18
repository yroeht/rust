/* https://leetcode.com/problems/longest-common-prefix/ 14 */

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = "".to_string();
        for (i, c) in strs[0].chars().enumerate() {
            for s in &strs {
                if s.chars().nth(i).unwrap_or('\0') != c {
                    return prefix
                }
            };
            prefix.push(c);
        };
        prefix
    }
}

fn main() {
    dbg!(Solution::longest_common_prefix(
            vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
            ]));
    dbg!(Solution::longest_common_prefix(
            vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
            ]));
}
