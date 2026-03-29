impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut prod, mut no_of_zeros) = (1, 0);
        for num in nums.iter() {
            if *num == 0 {
                no_of_zeros += 1;
            } else {
                prod *= *num;
            }
        }
        if no_of_zeros > 1 {
            return vec![0; nums.len()];
        } else if no_of_zeros == 1 {
            let mut ans: Vec<i32> = Vec::new();
            for num in nums.iter() {
                if *num != 0 {
                    ans.push(0);
                } else {
                    ans.push(prod);
                }
            }
            return ans;
        } else {
            let mut ans: Vec<i32> = Vec::new();
            for num in nums.iter() {
                ans.push(prod / *num);
            }
            return ans;
        }
    }
}