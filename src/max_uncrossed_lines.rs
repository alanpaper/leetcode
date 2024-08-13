/// 1035. 不相交的线
/// 在两条独立的水平线上按给定的顺序写下 nums1 和 nums2 中的整数。
/// 现在，可以绘制一些连接两个数字 nums1[i] 和 nums2[j] 的直线，这些直线需要同时满足：
/// nums1[i] == nums2[j]
/// 且绘制的直线不与任何其他连线（非水平线）相交。
/// 请注意，连线即使在端点也不能相交：每个数字只能属于一条连线。
/// 以这种方法绘制线条，并返回可以绘制的最大连线数。
///
pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut index_1 = 0;
    let mut index_1 = 0;
}

#[test]
fn test_1() {
    // 1 4 2
    // 1 2 4
    assert_eq!(max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
}
#[test]
fn test_2() {
    // 2, 5,1,2,5
    // 10,5,2,1,5,2
    assert_eq!(
        max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
}

#[test]
fn test_3() {
    // 1, 3, 7, 1, 7, 5
    // 1, 9, 2, 5, 1
    assert_eq!(
        max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}
