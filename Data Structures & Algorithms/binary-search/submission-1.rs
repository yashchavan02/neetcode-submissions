impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return -1; }
        
        let (mut l, mut r) = (0usize, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] == target { 
                return m as i32; 
            } else if nums[m] < target { 
                l = m + 1; 
            } else { 
                if m == 0 { break; }
                r = m - 1; 
            }
        }
        -1
    }
}