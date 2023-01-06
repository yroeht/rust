/* https://leetcode.com/problems/reverse-bits/ 190 */

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut acc = 0;
        for i in 0..32 {
            acc <<= 1;
            acc |= x & 1;
            x >>= 1
        }
        acc
    }
}
fn main() {
    dbg!(Solution::reverse_bits(0b00000010100101000001111010011100));
    dbg!(Solution::reverse_bits(0b11111111111111111111111111111101));
}
