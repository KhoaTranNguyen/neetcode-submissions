use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        let mut counts: HashMap<char, i32> = HashMap::new();

        // Pass 1: +1 for every character in `s`
        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        // Pass 2: -1 for every character in `t`
        for ch in t.chars() {
            let count = counts.entry(ch).or_insert(0);
            *count -= 1;
            // If count drops below 0, `t` has more of this character than `s`
            if *count < 0 {
                return false;
            }
        }
        true
    }
}
