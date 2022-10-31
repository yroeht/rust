/* https://leetcode.com/problems/roman-to-integer 13 */

/* Assumes input is a valid roman numeral. */
fn roman_to_int(s: String) -> i32 {
    let vals = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000)];
    let mut state = 0;
    let mut res = 0;
    for c in s.chars().rev() {
        for (idx, (numeral, value)) in vals.iter().enumerate() {
            if numeral == &c {
                if idx < state {
                    res -= value;
                } else {
                    res += value;
                    state = idx;
                }
                break;
            }
        }

    }
    res
}

fn main() {
    dbg!(roman_to_int("MCMXCIV".to_string()));
}
