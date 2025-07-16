use std::collections::HashMap;

/// 2374. 边积分最高的节点
/// 给你一个有向图，图中有 n 个节点，节点编号从 0 到 n - 1 ，其中每个节点都 恰有一条 出边。
/// 图由一个下标从 0 开始、长度为 n 的整数数组 edges 表示，其中 edges[i]
/// 表示存在一条从节点 i 到节点 edges[i] 的 有向 边。
/// 节点 i 的 边积分 定义为：所有存在一条指向节点 i 的边的节点的 编号 总和。
/// 返回 边积分 最高的节点。如果多个节点的 边积分 相同，返回编号 最小 的那个。

pub fn edge_score(edges: Vec<i32>) -> i32 {
    let mut arr = vec![0; edges.len()];
    for i in 0..edges.len() {
        arr[edges[i] as usize] += i;
    }
    let mut max = arr[0];
    let mut index = 0;
    for a in arr.iter().enumerate() {
        if max < *a.1 {
            max = *a.1;
            index = a.0;
        }
    }
    index as i32
}

#[test]
fn test_1() {
    assert_eq!(edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7)
}

#[test]
fn test_2() {
    assert_eq!(edge_score(vec![2, 0, 0, 2]), 0)
}
