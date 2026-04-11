impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n == 0 { return -1; }
        let (mut l, mut r) = (0usize, n - 1);

        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] == target { return m as i32; }

            if nums[l] <= nums[m] {
                if nums[l] <= target && target < nums[m] {
                    if m == 0 { break; }
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if nums[m] < target && target <= nums[r] {
                    l = m + 1;
                } else {
                    if m == 0 { break; }
                    r = m - 1;
                }
            }
        }
        -1
    }
}