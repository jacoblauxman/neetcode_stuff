// # 121 Best Time to Buy and Sell Stock

pub fn max_profit(prices: Vec<i32>) -> i32 {
    // initialize max for return, buy and sell pointers re: idx in `prices`
    let (mut max, mut buy, mut sell) = (0, 0, 1);

    // iter through prices until sell reaches end
    while sell < prices.len() {
        // valid time to buy+sell
        if prices[buy] < prices[sell] {
            // reassign max to greatest value between it and sell-buy `diff`
            max = std::cmp::max(max, prices[sell] - prices[buy]);
        } else {
            // invalid range of "dates" (no profit possible)
            // -> move buy date up to sell and continue checking
            buy = sell;
        }
        // move forward in prices
        sell += 1;
    }

    max
}
