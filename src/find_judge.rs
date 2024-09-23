/// 997. 找到小镇的法官
/// 小镇里有 n 个人，按从 1 到 n 的顺序编号。传言称，这些人中有一个暗地里是小镇法官。
// 如果小镇法官真的存在，那么：
// 小镇法官不会信任任何人。
// 每个人（除了小镇法官）都信任这位小镇法官。
// 只有一个人同时满足属性 1 和属性 2 。
// 给你一个数组 trust ，其中 trust[i] = [ai, bi] 表示编号为 ai 的人信任编号为 bi 的人。
// 如果小镇法官存在并且可以确定他的身份，请返回该法官的编号；否则，返回 -1 。

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {}

#[test]
fn test_1() {
    assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
}

#[test]
fn test_2() {
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
}

#[test]
fn test_3() {
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
}
