impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l_min = std::i32::MAX;
        let mut max_profit = 0i32;

        for num in prices.iter() {
            l_min = if *num < l_min { *num } else { l_min };
            let profit = num - l_min;
            max_profit = if  profit > max_profit { profit } else { max_profit };
        }

        max_profit
    }
}
