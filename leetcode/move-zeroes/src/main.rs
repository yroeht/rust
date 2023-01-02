/* https://leetcode.com/problems/move-zeroes/ 283 */

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&n| n != 0);
        nums.resize(len, 0)
    }
}

fn main() {
    let mut v = vec![0,1,0,3,12];
    dbg!(Solution::move_zeroes(&mut v));
    dbg!(v);
}
