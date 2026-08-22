impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency_table: HashMap<i32, u16> = HashMap::new();

        for num in nums {
            *frequency_table.entry(num).or_insert(0) += 1;
        }

        let mut pairs: Vec<(i32, u16)> = frequency_table.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));

        pairs.into_iter()
            .take(k as usize)
            .map(|(num, _count)| num)
            .collect()
    }
}
