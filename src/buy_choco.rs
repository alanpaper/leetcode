pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut prices_next = prices;
    prices_next.sort();

    let one = prices_next[0];
    let two = prices_next[1];

    if one + two <= money {
        return money - one - two;
    }

    money
}

#[test]
fn buy_choco_test() {
    assert_eq!(buy_choco(vec![1, 2, 2], 3), 0)
}

#[test]
fn buy_choco_test1() {
    assert_eq!(buy_choco(vec![3, 2, 3], 3), 3)
}
