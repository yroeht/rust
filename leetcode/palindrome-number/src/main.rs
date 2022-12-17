/* https://leetcode.com/problems/palindrome-number/ 9 */

struct Solution;

impl Solution {
    fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rx = 0;
        while x > 0 {
            rx = rx * 10 + x % 10;
            x /= 10
        };
        rx
    }
    pub fn is_palindrome(x: i32) -> bool {
        x >= 0 && x == Solution::reverse(x)
    }
}

fn main() {
    dbg!(Solution::is_palindrome(121));
    dbg!(Solution::is_palindrome(-121));
    dbg!(Solution::is_palindrome(10));
    dbg!(Solution::is_palindrome(1221));
}
