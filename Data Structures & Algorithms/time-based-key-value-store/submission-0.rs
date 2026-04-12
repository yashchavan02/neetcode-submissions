use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .or_insert(Vec::new())
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.map.get(&key) {
            let mut left = 0;
            let mut right = values.len() as i32 - 1;
            let mut res = "";

            while left <= right {
                let mid = left + (right - left) / 2;
                let (time, val) = &values[mid as usize];

                if *time <= timestamp {
                    res = val;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            return res.to_string();
        }

        "".to_string()
    }
}