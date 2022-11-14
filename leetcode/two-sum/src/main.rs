/* https://leetcode.com/problems/two-sum/ 1 */

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<&i32, i32> = HashMap::new();
        for (i, e) in nums.iter().enumerate() {
            if hm.contains_key(&(target - e)) {
                return vec![hm[&(target - e)], i as i32];
            }
            hm.insert(e, i as i32);
        };
        Vec::new()
    }
}
fn main() {
    dbg!(Solution::two_sum(vec![2,7,11,15], 9));
    dbg!(Solution::two_sum(vec![3,2,4], 6));
    dbg!(Solution::two_sum(vec![3,3], 6));
}
