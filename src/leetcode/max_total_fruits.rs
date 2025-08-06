/// 2106. 摘水果
/// 
pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
    let mut ans = 0;
    let mut right = start_pos + k;
    let mut left = 0; // 向左递减的位置
    ans
}


#[test]
fn test_1() {
    assert_eq!(max_total_fruits(vec![vec![2,8],vec![6,3],vec![8,6]], 5, 4), 9);
}


#[test]
fn test_3() {
    assert_eq!(max_total_fruits(vec![
        vec![0,3],
        vec![6,4],
        vec![8,5]
    ], 3, 2), 0);
}

#[test]
fn test_4() {
    assert_eq!(max_total_fruits(vec![
        vec![20000,10000],
    ], 20000, 0), 10000);
}

#[test]
fn test_6() {
    assert_eq!(max_total_fruits(vec![
        vec![0,10000],
    ], 20000, 20000), 10000);
}

#[test]
fn test_2() {
    assert_eq!(max_total_fruits(vec![
        vec![0,9],
        vec![4,1],
        vec![5,7],
        vec![6,2],
        vec![7,4],
        vec![10,9]
    ], 5, 4), 14);
}