impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        fn bisect_left(jobs: &Vec<(i32, i32, i32)>, val: i32) -> usize {
            let (mut l, mut r) = (0, jobs.len());
            while l < r {
                let m = l + (r - l) / 2;
                if jobs[m].0 >= val {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            l
        };

        fn dfs(i: usize, jobs: &Vec<(i32, i32, i32)>, table: &mut Vec<i32>) -> i32 {
            if i == jobs.len() { return 0; }
            if table[i] != -1 { return table[i]; }

            let skip = dfs(i + 1, jobs, table);
            let (_, e, p) = jobs[i];
            let j = bisect_left(jobs, e);
            let take = p + dfs(j, jobs, table);

            table[i] = take.max(skip);
            table[i]
        }

        let n = start_time.len();
        let (mut table, mut jobs) = (vec![-1; n], vec![]);
        for i in 0..n {
            jobs.push((start_time[i], end_time[i], profit[i]));
        }
        jobs.sort();
        dfs(0, &jobs, &mut table)
    }
}