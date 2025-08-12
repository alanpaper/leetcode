/// 2778. 
pub fn number_of_ways(n: i32, x: i32) -> i32 {
    let dp = &mut vec![0i64; (n + 1) as usize];
    dp[0] = 1;
    for i in 1..=n {
        let v = i.pow(x as u32);
        if v > n {
            break;
        }
        for s in (v..=n).rev() {
            dp[s as usize] += dp[(s - v) as usize];
        }
    }
    (dp[n as usize] % 1_000_000_007) as i32
}

#[test]
fn test_number_of_ways() {
    assert_eq!(number_of_ways(10, 2), 1);
    assert_eq!(number_of_ways(4, 1), 2);
}