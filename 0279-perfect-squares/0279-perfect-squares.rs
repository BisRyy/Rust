impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut res = vec![-1; n as usize];
        res[0] = 0;
        let mut que = std::collections::VecDeque::from(vec![(0, 0)]);
        while !que.is_empty() {
            let (val, step) = que.pop_front().unwrap();
            for i in 1..100 {
                let target = val + i * i;
                if target < n && res[target as usize] == -1 {
                    res[target as usize] = step + 1;
                    que.push_back((target, step + 1));
                }
                if target == n {
                    return step + 1;
                } else if target > n {
                    break;
                }
            }
        }
        return -1;

    }
}