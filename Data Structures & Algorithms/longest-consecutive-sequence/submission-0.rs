use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;
        for &val in set.iter() {
            if !set.contains(&(val - 1)) {
                let mut cnt = 1;
                let mut curr = val;
                while set.contains(&(curr + 1)) {
                    cnt += 1;
                    curr += 1;
                }
                ans = ans.max(cnt);
            }
        }
        ans
    }
}