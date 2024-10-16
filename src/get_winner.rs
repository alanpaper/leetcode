/// 1535. 找出数组游戏的赢家
/// 给你一个由 不同 整数组成的整数数组 arr 和一个整数 k 。
/// 每回合游戏都在数组的前两个元素（即 arr[0] 和 arr[1] ）之间进行。比较 arr[0] 与 arr[1] 的大小，
/// 较大的整数将会取得这一回合的胜利并保留在位置 0 ，较小的整数移至数组的末尾。当一个整数赢得 k 个连续回合时，游戏结束，该整数就是比赛的 赢家 。
/// 返回赢得比赛的整数。
/// 题目数据 保证 游戏存在赢家。
pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let mut pre_max = -1;
    for i in 0..arr.len() {
        if arr[i] > pre_max {
            let mut sub_total = 0;
            for j in (i + 1)..arr.len() {
                if arr[i] > arr[j] {
                    sub_total += 1;
                } else {
                    break;
                }
            }
            if i == 0 && sub_total >= k {
                return arr[i];
            }
            if i != 0 && sub_total + 1 >= k {
                return arr[i];
            }
        }
        pre_max = pre_max.max(arr[i]);
    }
    pre_max
}

#[test]
fn test_1() {
    assert_eq!(get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
}

// 2354671 1
// 3546712 1
// 5467123 1
// 5671234 2
// 6712345 1
// 7123456 1
// 7234561 2
// 7345612 3

#[test]
fn test_2() {
    assert_eq!(get_winner(vec![3, 2, 1], 10), 3);
}
#[test]
fn test_3() {
    assert_eq!(get_winner(vec![1, 9, 8, 2, 3, 7, 6, 4, 5], 7), 9);
}
#[test]
fn test_4() {
    assert_eq!(
        get_winner(vec![1, 11, 22, 33, 44, 55, 66, 77, 88, 99], 1000000000),
        99
    );
}
#[test]
fn test_5() {
    assert_eq!(
        get_winner(
            vec![
                904, 797, 25, 566, 643, 134, 400, 41, 835, 528, 165, 43, 282, 225, 857, 594, 149,
                703, 431, 899, 144, 941, 840, 16, 230, 403
            ],
            13
        ),
        904
    );
}
