/* https://leetcode.com/problems/ransom-note/ 383 */

use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut hm = HashMap::new();
    for c in magazine.chars() {
        hm.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    for c in ransom_note.chars() {
        let cnt =
            match hm.get_mut(&c) {
                Some(count) => count,
                _ => return false
            };
        if *cnt < 1 {
            return false;
        }
        *cnt -= 1;
    }
    true
}

fn main() {
    dbg!(can_construct("hhj".to_string(), "ahhkkj".to_string()));
    dbg!(can_construct("hhjj".to_string(), "ahhkkj".to_string()));
}
