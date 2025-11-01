impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for i in 0..prices.len() {
            let mut discount_applied = false;

            for j in (i + 1)..prices.len() {
                if prices[j] <= prices[i] {
                    result.push(prices[i] - prices[j]);
                    discount_applied = true;
                    break;
                }
            }

            if !discount_applied {
                result.push(prices[i]);
            }
        }

        result
    }
}
