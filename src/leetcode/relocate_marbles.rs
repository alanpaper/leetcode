use std::collections::HashSet;

/// 2766. 重新放置石块
pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
    let mut hashset = HashSet::new();
    for n in nums {
        hashset.insert(n);
    }
    for n in move_from.iter().enumerate() {
        if hashset.contains(&n.1) {
            hashset.remove(&n.1);
        }
        hashset.insert(move_to[n.0]);
    }
    let mut ans = hashset.into_iter().collect::<Vec<_>>();
    ans.sort();
    ans
}
