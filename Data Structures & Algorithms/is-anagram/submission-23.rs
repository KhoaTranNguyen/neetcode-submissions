use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        // Create an array of 26 atomic integers initialized to 0
        let counts: [AtomicI32; 26] = std::array::from_fn(|_| AtomicI32::new(0));
        thread::scope(|scope| {
            // Thread 1: +1 for characters in string `s`
            scope.spawn(|| {
                for b in s.bytes() {
                    let idx = (b - b'a') as usize;
                    counts[idx].fetch_add(1, Ordering::Relaxed);
                }
            });
            // Thread 2: -1 for characters in string `t`
            scope.spawn(|| {
                for b in t.bytes() {
                    let idx = (b - b'a') as usize;
                    counts[idx].fetch_sub(1, Ordering::Relaxed);
                }
            });
        }); // Both threads automatically join here before continuing
        // Verify all 26 atomic counters returned to 0
        counts.iter().all(|c| c.load(Ordering::Relaxed) == 0)
    }
}