/// 977. 有序数组的平方
/// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方
///  组成的新数组，要求也按 非递减顺序 排序。
///
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut min_index = 0;
}

#[test]
fn test_1() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}

#[test]
fn test_2() {
    assert_eq!(
        sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
