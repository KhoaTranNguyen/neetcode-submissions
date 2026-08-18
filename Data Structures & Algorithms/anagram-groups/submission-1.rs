use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_table: HashMap<[u16; 26], Vec<String>> = HashMap::new();

        for word in strs {
            let mut count = [0u16; 26];

            for b in word.bytes() {
                count[(b - b'a') as usize] += 1;
            }

            anagram_table.entry(count).or_insert(vec![]).push(word);
        }

        anagram_table.into_values().collect()
    }
}
