/// 2732. 找到矩阵中的好子集
/// 给你一个下标从 0 开始大小为 m x n 的二进制矩阵 grid 。
// 从原矩阵中选出若干行构成一个行的 非空 子集，如果子集中任何一列的和至多为子集大小的一半，那么我们称这个子集是 好子集。
// 更正式的，如果选出来的行子集大小（即行的数量）为 k，那么每一列的和至多为 floor(k / 2) 。
// 请你返回一个整数数组，它包含好子集的行下标，请你将其 升序 返回。
// 如果有多个好子集，你可以返回任意一个。如果没有好子集，请你返回一个空数组。
// 一个矩阵 grid 的行 子集 ，是删除 grid 中某些（也可能不删除）行后，剩余行构成的元素集合。
///
///
use std::collections::HashMap;
#[derive(Debug)]
struct BinaryMatrix {
    x: usize,
    h: Vec<i32>,
}

impl BinaryMatrix {
    fn new(x: usize, h: &Vec<i32>) -> Self {
        BinaryMatrix { x, h: h.clone() }
    }
}

pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let m = grid.len();
    if m == 1 {
        if grid[0].contains(&1) {
            return vec![];
        } else {
            return vec![0];
        }
    }
    let mut map: HashMap<i32, Vec<BinaryMatrix>> = HashMap::new();
    for x in grid.iter().enumerate() {
        map.entry(grid[x.0][0])
            .or_insert(Vec::new())
            .push(BinaryMatrix::new(x.0, x.1));
    }

    let mut ans = vec![];
    for i in 0..m {
        if grid[i][0] == 1 {
            if let Some(bm) = map.get(&0) {
                ans = subsetof_binary(bm, &grid, i);
                if !ans.is_empty() {
                    return ans;
                }
            }
        }
        if grid[i][0] == 0 {
            if let Some(bm) = map.get(&0) {
                ans = subsetof_binary(bm, &grid, i);
                if !ans.is_empty() {
                    return ans;
                }
            }
            if let Some(bm) = map.get(&1) {
                ans = subsetof_binary(bm, &grid, i);
                if !ans.is_empty() {
                    return ans;
                }
            }
        }

    }
    return vec![];
}

fn subsetof_binary(bm: &Vec<BinaryMatrix>, grid: &Vec<Vec<i32>>, index: usize) -> Vec<i32> {
    for b in bm {
        if good_subsetof_binary(&grid[index], &b.h) {
            if index == b.x {
                return vec![index as i32];
            }
            return vec![index as i32, b.x as i32];
        }
    }
    return vec![];
}

fn good_subsetof_binary(h1: &Vec<i32>, h2: &Vec<i32>) -> bool {
    let len = h1.len();
    let mut ans = true;
    for i in 0..len {
        if h1[i] + h2[i] > 1 {
            ans = false;
            break;
        }
    }
    ans
}

#[test]
fn test_0() {
    let grid = vec![vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 1, 1, 1]];
    assert_eq!(good_subsetof_binary_matrix(grid), vec![0, 1])
}

#[test]
fn test_1() {
    let grid = vec![vec![0]];
    assert_eq!(good_subsetof_binary_matrix(grid), vec![0])
}

#[test]
fn test_2() {
    let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
    assert_eq!(good_subsetof_binary_matrix(grid), vec![])
}

#[test]
fn test_3() {
    let grid = vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![1, 0]];
    assert_eq!(good_subsetof_binary_matrix(grid), vec![0])
}
