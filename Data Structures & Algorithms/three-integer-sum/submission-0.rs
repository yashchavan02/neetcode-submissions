impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        for idx in 0..n {
            if idx > 0 && nums[idx] == nums[idx - 1] { continue; }
            let (mut i, mut j) = (idx + 1, n - 1);
            while i < j {
                let sum = nums[idx] + nums[i] + nums[j];
                if sum == 0 {
                    ans.push(vec![nums[idx], nums[i], nums[j]]);
                    i += 1;
                    j -= 1;
                    while i < j && nums[i] == nums[i - 1] { i += 1; }
                    while i < j && nums[j] == nums[j + 1] { j -= 1; }
                } else if sum < 0 {
                    i += 1;
                } else {
                    j -= 1;
                }
            }
        }
        ans
    }
}