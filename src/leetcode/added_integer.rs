/// 3131. 找出与数组相加的整数 I
///
pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nums1_total = nums1.iter().min().unwrap();
    let nums2_total = nums2.iter().min().unwrap();
    nums2_total - nums1_total
}
/// 3132. 找出与数组相加的整数 II
/// 给你两个整数数组 nums1 和 nums2。
/// 从 nums1 中移除两个元素，并且所有其他元素都与变量 x 所表示的整数相加。如果 x 为负数，则表现为元素值的减少。
/// 执行上述操作后，nums1 和 nums2 相等 。当两个数组中包含相同的整数，并且这些整数出现的频次相同时，两个数组 相等 。
/// 返回能够实现数组相等的 最小 整数 x。
pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut nums_sort = nums1.clone();
    nums_sort.sort();
    let nums2_total = nums2.iter().min().unwrap();
    let mut ans = *nums2_total;
    for i in 0..3 {
        let min = *nums2_total - nums_sort[i];
        ans = ans.min(min);
    }
    ans
}

#[test]
fn test_1() {
    let nums1 = vec![4, 20, 16, 12, 8];
    let nums2 = vec![14, 18, 10];
    assert_eq!(minimum_added_integer(nums1.clone(), nums2.clone()), -2);
}
#[test]
fn test_2() {
    let nums1 = vec![3, 5, 5, 3];
    let nums2 = vec![7, 7];
    assert_eq!(minimum_added_integer(nums1.clone(), nums2.clone()), 2);
}
#[test]
fn test_3() {
    let nums1 = vec![4, 6, 3, 1, 4, 2, 10, 9, 5];
    let nums2 = vec![5, 10, 3, 2, 6, 1, 9];
    assert_eq!(minimum_added_integer(nums1.clone(), nums2.clone()), 0);
}
