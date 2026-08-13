use std::thread;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let (s_arr, t_arr) = thread::scope(|scope| {
            // Thread 1: Count string `s` in Core 1's isolated stack memory
            let handle_s = scope.spawn(|| {
                let mut counts = [0u16; 26];
                for b in s.bytes() {
                    counts[(b - b'a') as usize] += 1;
                }
                counts
            });

            // Thread 2: Count string `t` in Core 2's isolated stack memory
            let handle_t = scope.spawn(|| {
                let mut counts = [0u16; 26];
                for b in t.bytes() {
                    counts[(b - b'a') as usize] += 1;
                }
                counts
            });

            (handle_s.join().unwrap(), handle_t.join().unwrap())
        });

        // Compare the 2 separate arrays (1-instruction SIMD check)
        s_arr == t_arr
    }
}
