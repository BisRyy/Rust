impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = Vec::new();
        let mut f = 1;
        for i in 1..nums.len(){
            if nums[i] != nums[i-1]{
                ans.push(f);
                if f == 1{
                    return -1
                }
                f = 1;
            }else{
                f+=1;
            }
        }
        if f == 1{
            return -1
        }else{
            ans.push(f)
        }
        
        f=0;
        for i in ans{
            if i == 2 || i == 3{
                f+=1;
            }else if i%3==0{
                f+=i/3;
            }else{
                f+=i/3 + 1;
            }
        }
            
        return f as i32
        
    }
}

// 1 1 2 3 3 3 4 
// [2,2,2,2,3,3,3,4,4]
// 2 3 4 5 6 7 8 9 10 11
