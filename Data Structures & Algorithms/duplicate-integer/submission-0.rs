use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();  
        for val in nums.iter() {
            *map.entry(*val).or_insert(0) += 1;
        }
        for val in map.values() {
            if *val > 1 {
                return true;
            }
        }
        return false;
    }
}
