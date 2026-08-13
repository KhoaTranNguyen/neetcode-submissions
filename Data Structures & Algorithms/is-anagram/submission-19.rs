use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        let mut counts = [0i32; 26];

        for (byte_s, byte_t) in s.bytes().zip(t.bytes()) {
            counts[(byte_s - b'a') as usize] += 1;
            counts[(byte_t - b'a') as usize] -= 1;
        }

        counts.iter().all(|&v| v == 0)
    }
}
