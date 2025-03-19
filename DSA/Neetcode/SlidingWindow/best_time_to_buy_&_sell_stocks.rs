impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX ;
        let mut max_profit = 0 ;

        for &price in &prices {
            if price < min_price {
                min_price = price  ;
            }
            let mut potential_profit = price - min_price ;
            if potential_profit > max_profit {
                max_profit = potential_profit ;
            }
        }
        max_profit
    }
}