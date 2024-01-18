use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let (mut map, mut set) = (HashMap::new(), HashSet::new());
        arr.iter().for_each(|x| { map.entry(x).and_modify(|y| *y += 1).or_insert(1); });
        map.values().all(|v| set.insert(v))
    }
}