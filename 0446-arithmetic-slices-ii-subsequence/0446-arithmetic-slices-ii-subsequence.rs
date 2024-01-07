use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut vf = vec![vec![0; n]; n];
        let mut res = -((n*(n-1)/2) as i32);
        let mut revidx = HashMap::<i64, BTreeSet<usize>>::new();
        for i in 0..n {
            revidx.entry(nums[i] as i64).and_modify(|v| {v.insert(i);}).or_insert(BTreeSet::from([i]));
        }
        for i in 1..n {
            for j in 0..i {
                vf[j][i] = 1;
                if let Some(rids) = revidx.get(&(2i64 * nums[j] as i64 - nums[i] as i64)) {
                    vf[j][i] += rids.iter().filter_map(|&k| if k >= j {None} else {Some(vf[k][j])}).sum::<i32>()
                }
                res += vf[j][i];
            }
        }
        res

    }
}