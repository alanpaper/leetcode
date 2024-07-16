/// 3011. 判断一个数组是否可以变为有序
/// 给你一个下标从 0 开始且全是 正 整数的数组 nums 。
/// 一次 操作 中，如果两个 相邻 元素在二进制下数位为 1 的数目 相同 ，
/// 那么你可以将这两个元素交换。你可以执行这个操作 任意次 （也可以 0 次）。
/// 如果你可以使数组变有序，请你返回 true，否则返回 false。
pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
    nums.chunk_by_mut(|a, b| a.count_ones() == b.count_ones())
        .for_each(|chunk| chunk.sort_unstable());
    nums.windows(2).all(|w| w[1] >= w[0])
}

#[test]
fn test_1() {
    assert_eq!(can_sort_array(vec![8, 4, 2, 30, 15]), true);
}
