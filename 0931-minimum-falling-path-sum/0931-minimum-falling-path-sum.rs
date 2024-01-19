impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut res: Vec<Vec<i32>> = vec![vec![0;matrix[0].len()];matrix.len()];

        for row in 0..res.len(){
            for col in 0..matrix[row].len(){
                //Calculate the minimum cost to get to this spot.
                if row == 0 { res[row][col] = matrix[row][col]; continue; }
                let mut above_values: Vec<i32> = vec![];
                above_values.push(res[row-1][col]);

                if col - 1 < usize::MAX { above_values.push(res[row-1][col-1]); }
                if col + 1 < matrix[row].len(){ above_values.push(res[row-1][col+1]); }
                
                res[row][col] = matrix[row][col] + *(above_values.iter().min().unwrap());
            }
        }

        *(res[res.len()-1].iter().min().unwrap())
    }
}