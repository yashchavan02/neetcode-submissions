impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0i32, nums.len() as i32 - 1);
        while i < j {
            let sum = nums[i as usize] + nums[j as usize];
            if sum == target { return vec![i + 1, j + 1]; }
            else if sum < target { i += 1; }
            else { j -= 1; }
        }
        vec![]
    }
}
