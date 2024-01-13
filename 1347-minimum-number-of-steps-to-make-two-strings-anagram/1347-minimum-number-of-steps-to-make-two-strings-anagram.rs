use std::collections::HashMap;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut c: HashMap<char, usize> = HashMap::new();
        let mut d: HashMap<char, usize> = HashMap::new();

        for ch in s.chars() {
            *c.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            *d.entry(ch).or_insert(0) += 1;
        }

        let mut e = 0;

        for (i, count) in c.iter() {
            e += count.saturating_sub(*d.get(i).unwrap_or(&0));
        }

        e as i32
    }
}