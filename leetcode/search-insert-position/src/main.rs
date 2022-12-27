/* https://leetcode.com/problems/search-insert-position/ 35 */

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0
        }
        let pivot = (nums.len() - 1) / 2;
        if nums[pivot] == target {
            if pivot == 0 || nums[pivot - 1] != target {
                return pivot as i32
            }
            return Solution::search_insert(nums[0..(pivot - 1)].to_vec(), target)
        }
        if nums[pivot] < target {
            return pivot as i32 + 1 + Solution::search_insert(nums[(pivot + 1)..].to_vec(), target)
        }
        return Solution::search_insert(nums[0..pivot].to_vec(), target)
    }
}

fn main() {
    dbg!(Solution::search_insert(vec![1,3,5,6], 5));
    dbg!(Solution::search_insert(vec![1,3,5,6], 2));
    dbg!(Solution::search_insert(vec![1,3,5,6], 7));
}
