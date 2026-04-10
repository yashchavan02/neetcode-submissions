impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if nums[0usize] < nums[n - 1] { return nums[0usize]; }
        let (mut l, mut r) = (0usize, n - 1);
        let mut ans = i32::MAX;
        while l < r {
            let m = l + (r - l) / 2;
            if nums[l] <= nums[m] {
                ans = ans.min(nums[l]);
                l = m + 1;
            } else {
                ans = ans.min(nums[m]);
                r = m - 1;
            }
        }
        ans = ans.min(nums[l]);
        ans
    }
}
