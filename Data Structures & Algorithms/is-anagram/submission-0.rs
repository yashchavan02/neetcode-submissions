use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        } else {
            let mut map: HashMap<char, i32> = HashMap::new();
            for ch in s.chars() {
                *map.entry(ch).or_insert(0) += 1;
            }
            for ch in t.chars() {
                *map.entry(ch).or_insert(0) -= 1;
            }
            for val in map.values() {
                if *val != 0 {
                    return false;
                }
            }
            return true;
        } 
    }
}
