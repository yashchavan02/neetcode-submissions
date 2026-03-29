impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut ans = String::new();
        for s in strs {
            ans.push_str(&s.len().to_string());
            ans.push('#');
            ans.push_str(&s);
        }
        ans
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }
            let len: usize = s[i..j].parse().unwrap();
            let start = j + 1;
            let end = start + len;
            ans.push(s[start..end].to_string());
            i = end;
        }
        ans
    }
}