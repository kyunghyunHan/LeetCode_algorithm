impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
       let mut max_len = 0;   
      let mut cnt = 0;       

    for rect in rectangles {
        let side = rect[0].min(rect[1]);

        if side > max_len {
            max_len = side;
            cnt = 1;      
        } else if side == max_len {
            cnt += 1;    
        }
    }

    cnt
    }
}