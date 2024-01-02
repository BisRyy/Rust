impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        let mut freq = vec![0; nums.len() + 1];
        let mut ans = Vec::new();

        for &c in &nums {
            if freq[c as usize] >= ans.len() {
                ans.push(Vec::new());
            }

            ans[freq[c as usize]].push(c);
            freq[c as usize] += 1;
        }

        ans
        
    }
}
