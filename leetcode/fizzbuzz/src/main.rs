/* https://leetcode.com/problems/fizz-buzz/ 412 */

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for i in 1.. n + 1 {
        let mut x: String = String::new();
        if i % 3 == 0 {
            x += "Fizz";
        }
        if i % 5 == 0 {
            x += "Buzz";
        }
        if x.is_empty() {
            x.push_str(&i.to_string());
        }
        res.push(x);
    }
    res
}

fn main() {
    dbg!(fizz_buzz(20));
}
