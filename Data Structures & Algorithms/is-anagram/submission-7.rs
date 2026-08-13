use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        let mut counts: HashMap<char, [usize; 2]> = HashMap::new();

        // 1. Populate counts from the first string `s` (index 0)
        for ch in s.chars() {
            counts.entry(ch).or_insert([0, 0])[0] += 1;
        }
        // 2. Populate counts from the second string `t` (index 1)
        for ch in t.chars() {
            counts.entry(ch).or_insert([0, 0])[1] += 1;
        }


        counts.values().all(|[c1, c2]| c1 == c2)
    }
}
