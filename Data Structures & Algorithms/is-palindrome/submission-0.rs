impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch.is_alphanumeric() {
                stack.push(ch.to_ascii_lowercase());
            }
        }
        let (mut i, mut j) = (0i32, stack.len() as i32 - 1);
        while i < j {
            if stack[i as usize] != stack[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
