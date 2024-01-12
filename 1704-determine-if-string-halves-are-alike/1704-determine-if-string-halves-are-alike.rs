use std::collections::HashSet;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut x = 0;
        let mut c = 0;
        
        let mut y: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        for i in s.to_lowercase().chars(){
            if y.contains(&i){
                if c < s.len()/2{
                    x+=1;
                }else{
                    x-=1;
                }
            }
            c+=1;
        }
        x == 0
    }
}