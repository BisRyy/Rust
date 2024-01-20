impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        const MOD: i32 = 1000000007;
        let mut dp = vec![0; arr.len()];
        let mut stack = VecDeque::new();
        
        for i in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.back().unwrap() as usize] >= arr[i] {
                stack.pop_back();
            }

            if stack.is_empty() {
                dp[i] = (i + 1) as i32 * arr[i];
            } else {
                let j = *stack.back().unwrap() as i32;
                dp[i] = dp[j as usize] + (i as i32 - j) * arr[i];
            }

            stack.push_back(i as i32);
        }

        dp.iter().fold(0, |acc, v| (acc + v) % MOD) % MOD
    }
}