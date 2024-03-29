impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut a = nums;
        a.sort_unstable();

        let mut d = Vec::with_capacity(a.len());
        let mut p = Vec::with_capacity(a.len());

        for (i, &ai) in a.iter().enumerate() {
            let (dj, j) = a[..i]
                .iter()
                .enumerate()
                .filter(|&(_, &aj)| ai % aj == 0)
                .map(|(j, _)| (d[j], j))
                .max()
                .unwrap_or((0, i));
            d.push(dj + 1);
            p.push(j);
        }

        let pos = d.into_iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0;
        std::iter::successors(Some(pos), |&pos| Some(p[pos]).filter(|&x| x != pos))
            .map(|i| a[i])
            .collect()
    }
}