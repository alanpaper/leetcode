/// 292. Nim游戏
pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

#[test]
fn test_1() {
    assert_eq!(can_win_nim(4), false);
}
#[test]
fn test_2() {
    assert_eq!(can_win_nim(1), true);
}
#[test]
fn test_3() {
    assert_eq!(can_win_nim(2), true);
}
