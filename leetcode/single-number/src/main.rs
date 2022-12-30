/* https://leetcode.com/problems/single-number/ 136 */

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        'outer: for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue
                }
                if nums[i] == nums[j] {
                    continue 'outer
                }
            }
            return nums[i]
        }
        panic!()
    }
}

fn main() {
    dbg!(Solution::single_number(vec![2, 2, 1]));
    dbg!(Solution::single_number(vec![4, 1, 2, 1, 2]));
    dbg!(Solution::single_number(vec![1]));
}
