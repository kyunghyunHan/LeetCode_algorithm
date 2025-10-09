impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {

       arr.sort_by(|a, b| {
        let a_ones = format!("{:b}", a).chars().filter(|&c| c == '1').count();
        let b_ones = format!("{:b}", b).chars().filter(|&c| c == '1').count();

        a_ones.cmp(&b_ones).then(a.cmp(b)) 
    });

    arr

    }
}