use std::collections::{HashMap, HashSet};
/// 684. 冗余连接
/// 树可以看成是一个连通且 无环 的 无向 图。
/// 给定往一棵 n 个节点 (节点值 1～n) 的树中添加一条边后的图。
/// 添加的边的两个顶点包含在 1 到 n 中间，且这条附加的边不属于树中已存在的边。
/// 图的信息记录于长度为 n 的二维数组 edges ，edges[i] = [ai, bi] 表示图中在 ai 和 bi 之间存在一条边。
/// 请找出一条可以删去的边，删除后可使得剩余部分是一个有着 n 个节点的树。如果有多个答案，则返回数组 edges 中最后出现的那个。
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = HashMap::new();
    for e in &edges {
        map.entry(e[0]).or_insert(HashSet::new()).insert(e[1]);
    }
    vec![]
}

#[test]
fn test_1() {
    assert_eq!(
        find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
}
#[test]
fn test_2() {
    assert_eq!(
        find_redundant_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![1, 5]
        ]),
        vec![1, 4]
    );
}
