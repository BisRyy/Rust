impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();      
        for s in strs.into_iter() {       
            let mut key = [0; 26];
            for ch in s.chars() {
                key[(ch as u32 - 'a' as u32) as usize] +=1;
            }
            map.entry(key).or_insert(Vec::new()).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}