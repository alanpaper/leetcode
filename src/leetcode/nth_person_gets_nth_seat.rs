/// 1227. 飞机座位分配概率
/// 有 n 位乘客即将登机，飞机正好有 n 个座位。第一位乘客的票丢了，他随便选了一个座位坐下。
/// 剩下的乘客将会：
/// 如果他们自己的座位还空着，就坐到自己的座位上，
/// 当他们自己的座位被占用时，随机选择其他座位
/// 第 n 位乘客坐在自己的座位上的概率是多少？

pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
    if n > 1 {
        return 0.5;
    }
    return 1.0;
}

#[test]
fn test_1() {
    assert_eq!(nth_person_gets_nth_seat(1), 1.0)
}

#[test]
fn test_2() {
    assert_eq!(nth_person_gets_nth_seat(2), 0.5)
}
#[test]
fn test_3() {
    assert_eq!(nth_person_gets_nth_seat(3), 0.5)
}
