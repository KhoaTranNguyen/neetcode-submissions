use std::collections::HashMap;

impl Solution {

    fn count_characters_sorted(input: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for ch in input.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        counts
    }
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        let map_s = Self::count_characters_sorted(&s);
        let map_t = Self::count_characters_sorted(&t);

        map_s == map_t
    }
}
