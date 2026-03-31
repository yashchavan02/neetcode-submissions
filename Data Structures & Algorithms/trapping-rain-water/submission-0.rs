impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 { return 0; }

        let (mut left, mut right) = (0usize,n - 1);
        let (mut left_max, mut right_max) = (0i32, 0i32);

        let mut ans = 0i32;

        while left < right {
            if height[left] < height[right] {
                left_max = left_max.max(height[left]);
                ans += left_max - height[left];
                left += 1;
            } else {
                right_max = right_max.max(height[right]);
                ans += right_max - height[right]; 
                right -= 1;
            }
        }

        ans
    }
}