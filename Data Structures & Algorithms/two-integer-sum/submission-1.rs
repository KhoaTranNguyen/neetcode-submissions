use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut seen = HashMap::new();

        for (cur_idx, &num) in nums.iter().enumerate() {
            
            let complement = target - num;

            if let Some(&prev_idx) = seen.get(&complement) {
                return vec![prev_idx as i32, cur_idx as i32];
            }

            seen.insert(num, cur_idx);
        }

        vec![]
    }
}
