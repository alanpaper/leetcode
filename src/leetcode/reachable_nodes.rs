use std::collections::{HashMap, HashSet, VecDeque};

/// 2368. 受限条件下可到达节点的数目
///
pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let mut res_map = HashMap::new();
    let mut res_set = HashSet::new();
    let mut aleady_set = HashSet::new();
    for r in restricted {
        res_set.insert(r);
    }
    for node in edges {
        res_map.entry(node[0]).or_insert(Vec::new()).push(node[1]);
        res_map.entry(node[1]).or_insert(Vec::new()).push(node[0]);
    }
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut ans: i32 = 0;
    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            aleady_set.insert(node);
            ans += 1;
            if let Some(x) = res_map.get(&node) {
                for n in x {
                    if !res_set.contains(&n) && !aleady_set.contains(&n) {
                        queue.push_back(*n);
                    }
                }
            }
        }
    }
    ans
}

//       - 7     - 2
// 0 - 5 - 1 - 3 - 6 - 9
//           - 4 - 8

#[test]
fn test_1() {
    assert_eq!(
        reachable_nodes(
            10,
            vec![
                vec![4, 1],
                vec![1, 3],
                vec![1, 5],
                vec![0, 5],
                vec![3, 6],
                vec![8, 4],
                vec![5, 7],
                vec![6, 9],
                vec![3, 2]
            ],
            vec![2, 7]
        ),
        8
    );
}
#[test]
fn test_2() {
    assert_eq!(
        reachable_nodes(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 5],
                vec![0, 4],
                vec![3, 2],
                vec![6, 5]
            ],
            vec![4, 2, 1]
        ),
        3
    );
}
