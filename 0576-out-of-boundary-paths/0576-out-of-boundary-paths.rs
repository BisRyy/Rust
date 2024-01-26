impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i32 = (1e9 + 7.) as i32;

        let (m, n) = (m as usize, n as usize);
        let (x, y) = (start_row as usize, start_column as usize);

        let mut count = 0;

        let mut dp = vec![vec![0; n]; m];
        let mut temp = dp.clone();

        dp[x][y] = 1;

        for _ in 0..max_move {
            for i in 0..m {
                for j in 0..n {
                    count = [i == m - 1, j == n - 1, i == 0, j == 0]
                        .into_iter()
                        .map(|&b| (b as i32) * dp[i][j])
                        .fold(count, |s, v| (s + v) % MOD);

                    let updated_val = [
                        (i > 0).then(|| dp[i - 1][j]),
                        (j > 0).then(|| dp[i][j - 1]),
                        (i < m - 1).then(|| dp[i + 1][j]),
                        (j < n - 1).then(|| dp[i][j + 1]),
                    ]
                    .into_iter()
                    .filter_map(|&v| v)
                    .fold(0, |s, v| (s + v) % MOD);

                    temp[i][j] = updated_val;
                }
            }

            dp.swap_with_slice(&mut temp)
        }

        count
    }
}