impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let (text1, text2): (Vec<_>, Vec<_>) = (text1.into_bytes(), text2.into_bytes());
        let (mut prev, mut curr) = (vec![0; n + 1], vec![0; n + 1]);

        for i in 1..=m {
            for j in 1..=n {
                curr[j] = match text1[i - 1] == text2[j - 1] {
                    true  => prev[j - 1] + 1,
                    false => prev[j].max(curr[j - 1]),
                }
            }
            prev.swap_with_slice(&mut curr);
        }

        prev.pop().unwrap()
    }
}