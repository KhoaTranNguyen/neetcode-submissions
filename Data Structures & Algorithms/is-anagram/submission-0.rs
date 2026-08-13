use std::thread;
use std::collections::BTreeMap;

impl Solution {

    fn count_characters_sorted(input: &str) -> BTreeMap<char, usize> {
        let mut counts = BTreeMap::new();

        for ch in input.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        counts
    }
    
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

        let (map_s, map_t) = thread::scope(|scope| {
            let handle_s = scope.spawn(|| Self::count_characters_sorted(&s));
            let handle_t = scope.spawn(|| Self::count_characters_sorted(&t));

            (handle_s.join().unwrap(), handle_t.join().unwrap())
        });
        map_s == map_t
    }
}
