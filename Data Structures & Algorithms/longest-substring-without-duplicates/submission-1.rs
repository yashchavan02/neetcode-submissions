impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let sv: Vec<char> = s.chars().collect();
        let n = sv.len();
        if n == 0 { return 0; }
        let mut freq: Vec<i32> = vec![0; 95];
        let mut l = 0usize;
        let mut max_len = 0usize;
        for r in 0..n {
            let ri = sv[r] as usize - 32;
            freq[ri] += 1;
            while freq[ri] > 1 {
                freq[sv[l] as usize - 32] -= 1;
                l += 1;
            }
            max_len = max_len.max(r - l + 1);
        }
        max_len as i32
    }
}