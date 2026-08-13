use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        // Use i16 (signed) to allow negative values without underflowing!
        let mut s_arr = [0i16; 26];

        for (a, b) in s.bytes().zip(t.bytes()) {
            s_arr[(a - b'a') as usize] += 1;
            s_arr[(b - b'a') as usize] -= 1;
        }
        
        // Direct SIMD comparison with an array of 26 zeros!
        s_arr == [0i16; 26]
    }
}
