impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        for (i, &num_i) in nums.iter().enumerate() {
            let search_num = target - &num_i;

            for (j, &num_j) in nums.iter().enumerate().skip(i+1) {
                if search_num == num_j {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
}
