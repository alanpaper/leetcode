/// 3193. 统计逆序对的数目
/// 给你一个整数 n 和一个二维数组 requirements ，其中 requirements[i] = [endi, cnti] 表示这个要求中的末尾下标和 逆序对 的数目。
/// 整数数组 nums 中一个下标对 (i, j) 如果满足以下条件，那么它们被称为一个 逆序对 ：
/// i < j 且 nums[i] > nums[j]
/// 请你返回 [0, 1, 2, ..., n - 1] 的
/// 排列
/// perm 的数目，满足对 所有 的 requirements[i] 都有 perm[0..endi] 恰好有 cnti 个逆序对。
/// 由于答案可能会很大，将它对 109 + 7 取余 后返回。
pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
    
}

#[test]
fn test_1() {
    assert_eq!(number_of_permutations(3, vec![vec![2, 2], vec![0, 0]]), 2);
}

#[test]
fn test_2() {
    assert_eq!(
        number_of_permutations(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]]),
        3
    );
}

#[test]
fn test_3() {
    assert_eq!(number_of_permutations(3, vec![vec![0, 0], vec![1, 0]]), 1);
}
