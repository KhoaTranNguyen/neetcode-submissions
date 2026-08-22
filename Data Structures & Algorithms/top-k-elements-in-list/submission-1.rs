impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut frequency_table: HashMap<i32, u16> = HashMap::new();

        for num in nums {
            *frequency_table.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (num, freq) in frequency_table {
            heap.push(Reverse((freq, num)));
            if heap.len() > k {
                heap.pop();
            }
        }

        heap.into_iter().map(|Reverse((_, num))| num).collect()
    }
}
