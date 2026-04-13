impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let (m, n) = (a.len(), b.len());
        let mut low = 0;
        let mut high = m;

        while low <= high {
            let cut1 = (low + high) / 2;
            let cut2 = (m + n + 1) / 2 - cut1;

            let max_left1 = if cut1 == 0 { i32::MIN } else { a[cut1 - 1] };
            let min_right1 = if cut1 == m { i32::MAX } else { a[cut1] };

            let max_left2 = if cut2 == 0 { i32::MIN } else { b[cut2 - 1] };
            let min_right2 = if cut2 == n { i32::MAX } else { b[cut2] };

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                // correct partition
                if (m + n) % 2 == 0 {
                    return (max_left1.max(max_left2) as f64 
                          + min_right1.min(min_right2) as f64) / 2.0;
                } else {
                    return max_left1.max(max_left2) as f64;
                }
            } else if max_left1 > min_right2 {
                high = cut1 - 1;
            } else {
                low = cut1 + 1;
            }
        }

        0.0
    }
}