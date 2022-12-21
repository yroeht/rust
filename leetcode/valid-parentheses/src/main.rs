/* https://leetcode.com/problems/valid-parentheses/ 20 */

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                ')' | ']' | '}' => {
                    match stack.pop() {
                        None => return false,
                        Some(x) => match x {
                            '(' => if c != ')' { return false },
                            '[' => if c != ']' { return false },
                            '{' => if c != '}' { return false },
                            _ => panic!()
                        },
                    }
                },
                '(' | '[' | '{' => stack.push(c),
                _ => ()
            }
        };
        stack.len() == 0
    }
}

fn main() {
    dbg!(Solution::is_valid("()".to_string()));
    dbg!(Solution::is_valid("()[]{}".to_string()));
    dbg!(Solution::is_valid("(]".to_string()));
}
