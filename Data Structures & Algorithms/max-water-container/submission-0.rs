impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = heights.len() - 1;
        let mut ans = 0;
        while left < right {
            let width = (right - left) as i32;
            let height = heights[left].min(heights[right]);
            ans = ans.max(width * height);
            if heights[left] < heights[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}