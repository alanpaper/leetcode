// 62. 不同路径
// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。
// 问总共有多少条不同的路径？

// f[i][j] = f[i-1][j] + f[i][j-1]

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut f = vec![vec![1; n as usize]; m as usize];
    for i in 1..m {
        for j in 1..n {
            f[i as usize][j as usize] = f[i as usize - 1][j as usize] + f[i as usize][j as usize - 1];
        }
    }
    f[m as usize-1][n as usize - 1]
}



#[test]
fn test_1() {
    assert_eq!(unique_paths(3,7), 28);
}

#[test]
fn test_2() {
    assert_eq!(unique_paths(3, 3), 6);
}

#[test]
fn test_3() {
    assert_eq!(unique_paths(3, 2), 3);
}
