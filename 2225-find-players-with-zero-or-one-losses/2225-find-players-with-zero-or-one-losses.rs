use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut loss_count = HashMap::new();
        for m in matches {
            loss_count.entry(m[0]).or_insert(0);
            let count = loss_count.entry(m[1]).or_insert(0);
            *count += 1;
        }
        let mut v1: Vec<i32> = Vec::new();
        let mut v2: Vec<i32> = Vec::new();
        for (p, l) in loss_count {
            match l {
                0 => {
                    v1.push(p);
                }
                1 => {
                    v2.push(p);
                }
                _ => (),
            }
        }
        v1.sort();
        v2.sort();
        vec![v1, v2]
    }
}
