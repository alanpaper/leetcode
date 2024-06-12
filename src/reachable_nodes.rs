/// 2368. 受限条件下可到达节点的数目
///
pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let mut g = vec![vec![]; n as usize];

    for node in edges {
        g[node[0] as usize].push(node[1]);
        g[node[1] as usize].push(node[0]);
    }

    0
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
