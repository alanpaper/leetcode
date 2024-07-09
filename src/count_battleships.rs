/// 419. 甲板上的战舰
///
pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let m = board.len();
    let n = board[0].len();
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'X' {
                if i > 0 && board[i - 1][j] == 'X' {
                    continue;
                }
                if j > 0 && board[i][j - 1] == 'X' {
                    continue;
                }
                ans += 1;
            }
        }
    }
    ans
}

#[test]
fn test_1() {
    let board = vec![
        vec!['X', '.', '.', 'X'],
        vec!['.', '.', '.', 'X'],
        vec!['.', '.', '.', 'X'],
    ];
    assert_eq!(count_battleships(board), 2)
}
#[test]
fn test_2() {
    let board = vec![vec!['X']];
    assert_eq!(count_battleships(board), 1)
}
