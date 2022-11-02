/* https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/ 1337 */


struct Solution;

impl Solution {
    fn sum_row(row: &Vec<i32>) -> u32 {
        let mut cnt = 0;
        for e in row {
            if *e == 0 {
                break;
            }
            cnt = cnt + 1;
        }
        cnt
    }
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut rows = Vec::new();
        for (id, row) in mat.iter().enumerate() {
            rows.push((id as i32, Solution::sum_row(row)));
        }
        rows.sort_by(|a, b| a.1.cmp(&b.1));
        rows.into_iter()
            .map(|(id, _)| id)
            .take(k as usize)
            .collect()
    }
}



fn main() {
    let m = vec![
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0],
    ];
    dbg!(Solution::k_weakest_rows(m, 3));
}
