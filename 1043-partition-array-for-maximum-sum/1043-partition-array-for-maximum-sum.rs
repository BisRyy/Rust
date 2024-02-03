impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut memo = vec![-1; arr.len() + 1];
        memo[arr.len()] = 0;

        fn dp(start_idx: usize, memo: &mut Vec<i32>, arr: &Vec<i32>, k: i32) -> i32 {
            if memo[start_idx] > -1 {
                return memo[start_idx];
            }

            let (mut ret, mut win_max, mut sub_sum) = (0, 0, 0);
            for i in start_idx..(arr.len().min(start_idx + k as usize)) {
                win_max = win_max.max(arr[i]);
                sub_sum = (i - start_idx + 1) as i32 * win_max;
                ret = ret.max(sub_sum + dp(i + 1, memo, arr, k));
            }
            memo[start_idx] = ret;
            memo[start_idx]
        }

        dp(0, &mut memo, &arr, k)
 
    }
}