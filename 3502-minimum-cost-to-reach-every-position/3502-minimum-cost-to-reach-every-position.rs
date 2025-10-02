impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut result = vec![cost[0]];
        let mut c = cost[0];
        for i in 1..cost.len(){
            if cost[i] < c {
                c  = cost [i];
                result.push(c)
            }else{
                result.push(c)
            }
        }
        result
    }
}