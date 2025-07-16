use std::collections::HashMap;

/// 3175. 找到连续赢k场比赛的第一位玩家
/// 有 n 位玩家在进行比赛，玩家编号依次为 0 到 n - 1 。
/// 给你一个长度为 n 的整数数组 skills 和一个 正 整数 k ，其中 skills[i] 是第 i 位玩家的技能等级。skills 中所有整数 互不相同 。
/// 所有玩家从编号 0 到 n - 1 排成一列。
/// 比赛进行方式如下：
/// 队列中最前面两名玩家进行一场比赛，技能等级 更高 的玩家胜出。
/// 比赛后，获胜者保持在队列的开头，而失败者排到队列的末尾。
/// 这个比赛的赢家是 第一位连续 赢下 k 场比赛的玩家。

// 请你返回这个比赛的赢家编号。
pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let mut wanning = HashMap::new();
    let mut max = 0;
    let mut current = 0;
    for i in 1..skills.len() {
        if skills[max] < skills[i] {
            max = i;
        }
        if skills[current] < skills[i] {
            current = i;
        }
        wanning
            .entry(current)
            .and_modify(|f| {
                *f += 1;
            })
            .or_insert(1);
        if let Some(w) = wanning.get(&current) {
            if *w == k {
                return current as i32;
            }
        }
    }
    return max as i32;
}

#[test]
fn test_1() {
    assert_eq!(find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
}

#[test]
fn test_2() {
    assert_eq!(find_winning_player(vec![2, 5, 4], 3), 1);
}

#[test]
fn test_3() {
    assert_eq!(find_winning_player(vec![3, 10, 19, 2, 16, 14, 8, 17], 5), 2);
}
