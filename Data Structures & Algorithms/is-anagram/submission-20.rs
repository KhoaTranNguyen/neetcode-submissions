use std::collections::BTreeMap;

impl Solution {
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        // Stack-allocated 52-byte arrays (Zero Heap Overhead!)
        let mut s_arr = [0u16; 26];
        let mut t_arr = [0u16; 26];

        for (a, b) in s.bytes().zip(t.bytes()) {
            s_arr[(a - b'a') as usize] += 1;
            t_arr[(b - b'a') as usize] += 1;
        }
        
        s_arr == t_arr // 1-instruction SIMD vector compare
    }
}
