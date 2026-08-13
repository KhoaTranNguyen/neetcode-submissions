use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

            if s.len() != t.len() {
            return false;
        }
        let mut counts: HashMap<char, [usize; 2]> = HashMap::new();

        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            counts.entry(ch_s).or_insert([0, 0])[0] += 1;
            counts.entry(ch_t).or_insert([0, 0])[1] += 1;
        }
        
        counts.values().all(|[c1, c2]| c1 == c2)
    }
}
