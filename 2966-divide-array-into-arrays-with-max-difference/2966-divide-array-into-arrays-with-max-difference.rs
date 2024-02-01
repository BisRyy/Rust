impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut answer = vec![];
        
        for arr in nums.chunks(3).map(Vec::from) {
            if arr[2] - arr[0] > k {
                return vec![];
            }
            answer.push(arr)
        }

        answer 
    }
}