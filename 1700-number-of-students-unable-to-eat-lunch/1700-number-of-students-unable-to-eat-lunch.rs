impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut cnt0 = 0;
        let mut cnt1 = 0;

        for student in students{
            if student ==0 {
                cnt0 +=1;
            }else{
                cnt1 +=1;
            }
        }

        for sandwich in sandwiches{
            if sandwich ==0 && cnt0 > 0{
                cnt0-=1;
            }else if sandwich ==1 && cnt1 > 0 {
                cnt1 -= 1;
            }else {
                break;
            }
        }

        cnt0 + cnt1
    }
}