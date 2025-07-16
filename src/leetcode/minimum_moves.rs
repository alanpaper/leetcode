/// 3086.拾起K个1需要最少行动次数
/// 给你一个下标从 0 开始的二进制数组 nums，其长度为 n ；另给你一个 正整数 k 以及一个 非负整数 maxChanges 。
// Alice 在玩一个游戏，游戏的目标是让 Alice 使用 最少 数量的 行动 次数从 nums 中拾起 k 个 1 。游戏开始时，Alice 可以选择数组 [0, n - 1]
// 范围内的任何索引 aliceIndex 站立。如果 nums[aliceIndex] == 1 ，Alice 会拾起一个 1 ，并且 nums[aliceIndex] 变成0（这 不算 作一次行动）。
// 之后，Alice 可以执行 任意数量 的 行动（包括零次），在每次行动中 Alice 必须 恰好 执行以下动作之一：
// 选择任意一个下标 j != aliceIndex 且满足 nums[j] == 0 ，然后将 nums[j] 设置为 1 。这个动作最多可以执行 maxChanges 次。
// 选择任意两个相邻的下标 x 和 y（|x - y| == 1）且满足 nums[x] == 1, nums[y] == 0 ，然后交换它们的值（将 nums[y] = 1 和 nums[x] = 0）。
// 如果 y == aliceIndex，在这次行动后 Alice 拾起一个 1 ，并且 nums[y] 变成 0 。
// 返回 Alice 拾起 恰好 k 个 1 所需的 最少 行动次数。

/// 由题目的规则很容易看出来，maxChanges 肯定是放在我们当前位置的旁边，然后如果有位置是 1 的话，
/// 那么我们的位置也必定要在 1 的位置上，这样可以省去一次拿取的活动，所以最后我们要确定的 k 的个数其实是
/// temp = k-1-maxChanges，最后的答案最需要加上 maxChanges*2 即可
/// 然后我们遍历数组，使用另外一个数组来保存 1 的位置，然后遍历这些 1 的位置（也就是我们要固定待着的位置），
/// 并用两个左右指针维持窗口大小，窗口内的 1 保证是 temp 需要的个数即可，然后每次移动就维持好左右指针，
/// 尽量让左右指针到我们当前位置最小，然后和答案取最小值即可

pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
    0
}

#[test]
fn test_1() {
    let nums = vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1];
    assert_eq!(minimum_moves(nums, 3, 1), 3);
}

#[test]
fn test_2() {
    let nums = vec![0, 0, 0, 0];
    assert_eq!(minimum_moves(nums, 2, 3), 4);
}
