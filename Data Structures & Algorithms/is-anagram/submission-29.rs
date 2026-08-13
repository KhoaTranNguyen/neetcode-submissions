use std::collections::BTreeMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false
        }

        let mut s_vec = vec![0u16;26];
        let mut t_vec = vec![0u16;26];

        let origin = 'a' as usize;

        for (a,b) in s.chars().zip(t.chars()) {
            s_vec[a as usize - origin] +=1;
            t_vec[b as usize - origin] +=1;
        }

        s_vec == t_vec
    }
}

