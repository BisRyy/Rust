impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        
        let mut ans : usize = 0;
        let mut arr: Vec<usize> = Vec::new();
        for i in 0..bank.len() {
            let x = bank[i].matches("1").count();
            if x > 0 {
                arr.push(x);
            }
        }

        if arr.len() > 1 {
            for i in 1..arr.len() {
                ans += arr[i] * arr[i - 1];
            }
        }
        ans as i32
    }
}