impl Solution {
    pub fn is_open_bracket(ch: &char) -> bool {
        matches!(ch, '{' | '[' | '(')
    }

    pub fn is_matching_bracket(ch1: &char, ch2: &char) -> bool {
        matches!((ch1, ch2), ('{','}') | ('(',')') | ('[',']'))
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars() {
            if Self::is_open_bracket(&ch) {
                stack.push(ch);
            } else {
                if stack.is_empty() { return false; }
                let top = stack.pop().unwrap();
                if !Self::is_matching_bracket(&top, &ch) {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}