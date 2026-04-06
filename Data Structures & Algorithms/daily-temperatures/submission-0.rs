use std::collections::VecDeque;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut stack: VecDeque<usize> = VecDeque::new();
        let mut ans: Vec<i32> = vec![0; n];
        
        for i in 0..n {
            while !stack.is_empty() && temperatures[i] > temperatures[*stack.front().unwrap()] {
                let idx = *stack.front().unwrap();
                stack.pop_front().unwrap();
                ans[idx] = (i - idx) as i32;
            }
            stack.push_front(i);
        }
        ans
    }
}
