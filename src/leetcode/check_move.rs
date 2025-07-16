/// 1958. 检查操作是否合法
pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
    let list = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for n in list {
        let mut queue = vec![];

        let mut r_index = r_move;
        let mut c_index = c_move;
    }

    false
}

#[test]
fn test_1() {
    let board = vec![
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
    ];
    let r_move = 4;
    let c_move = 3;
    let color = 'B';
    assert_eq!(check_move(board, r_move, c_move, color), true);
}
