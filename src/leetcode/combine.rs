/// 77. 组合
/// 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
/// 你可以按 任何顺序 返回答案。

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {

    let mut ans = vec![];

    let mut path = vec![];
    fn dfs(start: i32, n: i32, k: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if path.len() == k as usize {
            ans.push(path.to_vec());
            return;
        }
        for i in start..=n {
            path.push(i);
            dfs(i + 1, n, k, path, ans);
            path.pop();
        }
    }
    dfs(1, n, k, &mut path, &mut ans);
    ans
}



pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut path  = vec![];

    fn dfs(index: usize, path: &mut Vec<i32>, nums: Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            ans.push(path.to_vec());
            return;
        }
        path.push(nums[index]);
        dfs(index + 1, path, nums.clone(), ans);
        path.pop();
        dfs(index + 1, path, nums, ans);
    }
    dfs(0, &mut path, nums, &mut ans);
    ans
}


#[test]
fn test_1() {
    assert_eq!(subsets(vec![1,2,3]), vec![
        vec![],
        vec![1],
        vec![2],
        vec![1,2],
        vec![3],
        vec![1,3],
        vec![2,3],
        vec![1,2,3]
        ])
}

#[test]
fn test_2() {
    assert_eq!(combine(4,2), vec![
        vec![2,4],
        vec![3,4],
        vec![2,3],
        vec![1,2],
        vec![1,3],
        vec![1,4],
        ])
}


