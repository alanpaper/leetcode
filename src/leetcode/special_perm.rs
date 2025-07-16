use std::collections::{HashMap, HashSet, VecDeque};
/// 2741. 特别的排列
/// 给你一个下标从 0 开始的整数数组 nums ，它包含 n 个 互不相同 的正整数。如果 nums 的一个排列满足以下条件，我们称它是一个特别的排列：
/// 对于 0 <= i < n - 1 的下标 i ，要么 nums[i] % nums[i+1] == 0 ，要么 nums[i+1] % nums[i] == 0 。
/// 请你返回特别排列的总数目，由于答案可能很大，请将它对 109 + 7 取余 后返回。
///
pub fn special_perm(nums: Vec<i32>) -> i32 {
    let mut edge = HashMap::new();
    let mut edge_all = HashMap::new();
    let len = nums.len();

    let mut queue = vec![];

    let mut node = HashSet::new();
    for i in 0..len {
        for j in (i + 1)..len {
            if nums[i] % nums[j] != 0 && nums[j] % nums[i] != 0 {
                queue.push((nums[i], nums[j]));
                node.insert(nums[i]);
                edge.entry(nums[i]).or_insert(Vec::new()).push(nums[j]);

                edge_all.entry(nums[i]).or_insert(Vec::new()).push(nums[j]);
                edge_all.entry(nums[j]).or_insert(Vec::new()).push(nums[i]);
            }
        }
    }

    println!("{:?}", edge_all);

    let total = factorial(len as i32);
    let mut sub_total = (queue.len() * 2) as u128 * (total / len as u128);

    let mut sub_total_repeat = 0;
    for n in node.iter() {
        if let Some(x) = edge_all.get(n) {
            for y in x {
                let min = edge_all.get(y).unwrap();
                sub_total_repeat += min.len() as u128 * factorial((len - 3) as i32);
            }
        }
    }
    println!("sub_total_repeat = {:?}", sub_total_repeat);
    println!("sub_total = {:?}", sub_total);

    // let res = total - (ans.len() * 2) as u128 * sub_total;
    // (res % 1000000007) as i32

    0
}

fn factorial(n: i32) -> u128 {
    let mut total: u128 = 1;
    for i in 0..n {
        total *= (i + 1) as u128;
    }
    total
}

fn dfs(edge: &HashMap<i32, Vec<i32>>, n: &i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut stack = VecDeque::new();
    stack.push_back((n, vec![*n]));

    while let Some((node, path)) = stack.pop_back() {
        let link = edge.get(node).unwrap();
        let new_link = link
            .iter()
            .filter(|f| !path.contains(f))
            .collect::<Vec<&i32>>();
        for child in &new_link {
            let mut new_path = path.clone();
            new_path.push(**child);
            if new_path.len() > 3 {
                result.push(new_path.clone());
            }
            stack.push_back((child, new_path));
        }
    }
    result
}

// #[test]
// fn test_1() {
//     assert_eq!(special_perm(vec![2, 3, 6]), 2);
// }

// #[test]
// fn test_2() {
//     assert_eq!(special_perm(vec![1, 4, 3]), 2);
// }

#[test]
fn test_3() {
    assert_eq!(special_perm(vec![20, 100, 50, 5, 10, 70, 7]), 48);
}

// 50 20 开头重复的
// 链 20 7
// 链 20 70
// 链 20 50

// 20 7 开头重复的
// 链 7 100
// 链 7 50
// 链 7 5
// 链 7 10
