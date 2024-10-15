/// 2806. 取整购买后的账户余额
pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
    let num = (100 - purchase_amount) % 10;
    if num <= 5 {
        return 100 - purchase_amount - num;
    }
    return 110 - purchase_amount - num;
}

#[test]
fn test_1() {
    assert_eq!(account_balance_after_purchase(9), 90);
}

#[test]
fn test_2() {
    assert_eq!(account_balance_after_purchase(15), 80);
}
