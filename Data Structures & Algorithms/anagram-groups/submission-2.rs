use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_table: HashMap<[u16; 26], Vec<String>> = HashMap::new();

        for word in strs {

            // for b in word.bytes() {
            //     count[(b - b'a') as usize] += 1;
            // }

            let count = word.bytes().fold([0u16; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                acc
            });

            anagram_table.entry(count).or_insert(vec![]).push(word);
        }

        anagram_table.into_values().collect()
    }
}
