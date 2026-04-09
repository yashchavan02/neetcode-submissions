impl Solution {
    fn find_hrs(piles: &Vec<i32>, k: i32) -> i32 {
        let mut total = 0;
        for &num in piles.iter() {
            total += (num + k - 1) / k; 
        }
        total
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max_k = *piles.iter().max().unwrap();
        let (mut l, mut r) = (1, max_k);

        while l < r {
            let m = l + (r - l) / 2;
            let hrs = Self::find_hrs(&piles, m);
            if hrs <= h {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}