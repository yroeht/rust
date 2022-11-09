/* https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/ 1342 */

struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return 0;
        };
        1 + Solution::number_of_steps(
            match num % 2 {
                0 => num / 2,
                _ => num - 1
            })
    }
}

fn main() {
    dbg!(Solution::number_of_steps(123));
}
