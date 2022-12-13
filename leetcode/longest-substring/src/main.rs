/* https://leetcode.com/problems/longest-substring-without-repeating-characters 3 */

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32
        };
        let mut hs = HashSet::new();
        for c in s.chars() {
            if hs.contains(&c) {
                break;
            };
            hs.insert(c);
        };
        let a = hs.len();
        if a > s.len() - 1 {
            return a as i32
        };
        let b = Solution::length_of_longest_substring(s[1..].to_string());
        if a < b as usize {
            b
        } else {
            a as i32
        }
    }
}

fn main() {
    dbg!(Solution::length_of_longest_substring("abcabcbb".to_string()));
    dbg!(Solution::length_of_longest_substring("bbbbb".to_string()));
    dbg!(Solution::length_of_longest_substring("pwwkew".to_string()));
    dbg!(Solution::length_of_longest_substring(" ".to_string()));
    dbg!(Solution::length_of_longest_substring("dvdf".to_string()));
}
