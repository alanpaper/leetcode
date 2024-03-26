pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut dp = vec![0; (amount + 1) as usize];
    dp[0] = 1;
    for coin in coins {
        for num in coin..=amount {
            dp[num as usize] += dp[(num - coin) as usize]
        }
    }
    dp[amount as usize]
}

#[test]
fn test_1() {
    assert_eq!(change(5, vec![1, 2, 5]), 5);
}
