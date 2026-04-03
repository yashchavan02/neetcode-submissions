impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() { return "".to_string(); }

        let mut freq = [0; 128];

        for ch in t.chars() { freq[ch as usize] += 1; }

        let sx: Vec<char> = s.chars().collect();

        let (mut left, mut right) = (0, 0);
        let mut count = t.len();
        let mut min_len = usize::MAX;
        let mut start = 0;

        while right < sx.len() {
            let r = sx[right] as usize;

            if freq[r] > 0 { count -= 1; }
            freq[r] -= 1;
            right += 1;
            
            while count == 0 {
                if right - left < min_len {
                    min_len = right - left;
                    start = left;
                }

                let l = sx[left] as usize;
                freq[l] += 1;

                if freq[l] > 0 {
                    count += 1;
                }

                left += 1;
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            sx[start..start + min_len].iter().collect()
        }
    }
}