/// 2225. 找出输掉零场或一场比赛的玩家
/// 给你一个整数数组 matches 其中 matches[i] = [winneri, loseri] 表示在一场比赛中 winneri 击败了 loseri 。
// 返回一个长度为 2 的列表 answer ：
// answer[0] 是所有 没有 输掉任何比赛的玩家列表。
// answer[1] 是所有恰好输掉 一场 比赛的玩家列表。
// 两个列表中的值都应该按 递增 顺序返回。
// 注意：
// 只考虑那些参与 至少一场 比赛的玩家。
// 生成的测试用例保证 不存在 两场比赛结果 相同 。
use std::collections::{HashMap, HashSet};

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut queue = HashSet::new();
    let mut queue_map = HashMap::new(); // 只输一次的
    for m in matches {
        queue.insert(m[0]);
        queue.insert(m[1]);
        queue_map.entry(m[1]).and_modify(|f| *f += 1).or_insert(1);
    }

    let mut zero = vec![];
    let mut one = vec![];
    for q in queue {
        if let Some(p) = queue_map.get(&q) {
            if *p == 1 {
                one.push(q);
            }
        }
        if !queue_map.contains_key(&q) {
            zero.push(q);
        }
    }
    zero.sort();
    one.sort();
    vec![zero, one]
}

#[test]
fn test_1() {
    assert_eq!(
        find_winners(vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9]
        ]),
        vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
    )
}
#[test]
fn test_2() {
    assert_eq!(
        find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]),
        vec![vec![1, 2, 5, 6], vec![]]
    )
}
