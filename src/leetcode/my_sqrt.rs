/// 69. x的平方根
///
pub fn my_sqrt(x: i32) -> i32 {
    let mut left = 0;
    let mut right = x;
    while left <= right {
        let mid = (right + left) / 2;
        if mid as i64 * mid as i64 <= x as i64 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left - 1
}

#[test]
fn test_1() {
    assert_eq!(my_sqrt(8), 2);
}
#[test]
fn test_2() {
    assert_eq!(my_sqrt(4), 2);
}
