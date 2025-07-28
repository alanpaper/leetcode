use std::collections::{HashMap};



/// 46. 全排列
/// 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut nums_copy = nums.clone();
    dfs(0, &mut nums_copy, &mut ans);
    ans
}


fn dfs(index: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        ans.push(nums.clone());
        return ;
    }
    for i in index..nums.len() {
        nums.swap(index, i);
        dfs(index + 1, nums,  ans);
        nums.swap(index, i);
    }
}

fn dfs_i(index: usize, nums: &mut Vec<i32>, unique: &mut HashMap<String, Vec<i32>>) {
    if index == nums.len() {
        let nums_str = nums.clone().iter().map(|f| format!("{}", f)).collect::<Vec<String>>().join(",");
        unique.insert(nums_str, nums.clone());
        return ;
    }
    for i in index..nums.len() {
        nums.swap(index, i);
        dfs_i(index + 1, nums,  unique);
        nums.swap(index, i);
    }
}
/// 47. 全排列II
/// 给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut ans_unique = HashMap::new();
    let mut nums_copy = nums.clone();
    dfs_i(0, &mut nums_copy, &mut ans_unique);
    for result in ans_unique.iter() {
        ans.push(result.1.to_vec());
    }
    ans
}

/// 47. 全排列II
/// 给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。
pub fn permute_unique_i(nums: Vec<i32>) -> Vec<Vec<i32>> {


    vec![]
}

fn backtrack() {

}


#[test]
fn test_1() {
    assert_eq!(permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]])
}

#[test]
fn test_2() {
    assert_eq!(permute(vec![1]), vec![vec![1]])
}


#[test]
fn test_3() {
    assert_eq!(permute_unique(vec![1,1,2]), vec![
        vec![1,1,2],
        vec![1,2,1],
        vec![2,1,1]
    ])
}

#[test]
fn test_4() {
    assert_eq!(permute_unique_i(vec![1,1,2]), vec![
        vec![1,1,2],
        vec![1,2,1],
        vec![2,1,1]
    ])
}


