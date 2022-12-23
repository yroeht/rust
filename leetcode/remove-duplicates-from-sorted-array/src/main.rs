/* https://leetcode.com/problems/remove-duplicates-from-sorted-array/ 26 */

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != nums[k] {
                k += 1;
                nums[k] = nums[i]
            }
        };
        (k + 1) as i32
    }
}

fn main() {
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    dbg!(Solution::remove_duplicates(&mut v));
    dbg!(v);
    let mut v = vec![1, 1, 2];
    dbg!(Solution::remove_duplicates(&mut v));
    dbg!(v);
}
