use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        let mut counts: HashMap<char, i32> = HashMap::new();

        // Increment for s, decrement for t simultaneously
        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            *counts.entry(ch_s).or_insert(0) += 1;
            *counts.entry(ch_t).or_insert(0) -= 1;
        }

        // Valid anagram if all net counts are 0
        counts.values().all(|&count| count == 0)
    }
}
