use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        
        if n == 0 || k == 0 || n < k { return vec![]; }

        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..n {
            if let Some(&front) = deque.front() {
                if front + k <= i { deque.pop_front(); }
            }
            while let Some(&back) = deque.back() {
                if nums[back] < nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);
            if i >= k - 1 { ans.push(nums[*deque.front().unwrap()]); }
        }
        ans
    }
}