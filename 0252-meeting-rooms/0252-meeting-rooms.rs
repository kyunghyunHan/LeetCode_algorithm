impl Solution {
pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
    intervals.sort_by_key(|x| x[0]);

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            return false;
        }
    }

    true
}

}