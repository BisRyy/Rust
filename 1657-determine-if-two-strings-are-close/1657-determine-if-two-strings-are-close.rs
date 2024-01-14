impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
      let mut memo1 = vec![0;26];
      let mut memo2 = vec![0;26];

      for c in word1.chars() {
        memo1[(c as u8 - 'a' as u8) as usize] += 1;
      }
      for c in word2.chars() {
        memo2[(c as u8 - 'a' as u8) as usize] += 1;
      }

      for i in 0..26 {
        let v1 = memo1[i];
        let v2 = memo2[i];
        if v1 != 0 && v2 == 0{
          return false
        } else if v1 == 0 && v2 != 0 {
          return false
        }
      }

      memo1.sort();
      memo2.sort();
      for i in 0..26 {
        if memo1[i] != memo2[i] {
          return false
        }
      }
      true
    }
}