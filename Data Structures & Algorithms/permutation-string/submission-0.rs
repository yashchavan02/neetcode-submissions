impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let sn1 = s1.len();
        let sn2 = s2.len();

        if sn1 > sn2 { return false; }

        let mut freq1 = vec![0; 26];
        let mut freq2 = vec![0; 26];

        for i in 0..sn1 {
            freq1[(s1.as_bytes()[i] - b'a') as usize] += 1;
            freq2[(s2.as_bytes()[i] - b'a') as usize] += 1;
        }

        if freq1 == freq2 { return true; }

        for i in sn1..sn2 {
            freq2[(s2.as_bytes()[i] - b'a') as usize] += 1;
            freq2[(s2.as_bytes()[i - sn1] - b'a') as usize] -= 1;
            if freq1 == freq2 { return true; }
        }

        false
    }
}