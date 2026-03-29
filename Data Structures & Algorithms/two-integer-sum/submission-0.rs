use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let comp = target - *num;
            if let Some(&indx) = map.get(&comp) {
                return vec![indx as i32, idx as i32];
            }
            map.insert(*num, idx as i32);
        }
        vec![]
    }
}