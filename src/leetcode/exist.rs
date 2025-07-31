use std::collections::{HashMap, HashSet};

/// 79. 单词搜索
/// 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。
/// 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {

    let m = board.len();
    let n = board[0].len();

    let mut board_map = HashMap::new();

    let mut word_set = HashSet::new();
    for w in word.chars().into_iter() {
        word_set.insert(w);
    }

    for i in 0..m {
        for j in 0..n  {
            if word_set.contains(&board[i][j]) {
                board_map.entry(&board[i][j]).and_modify(|f| {
                    *f += 1;
                }).or_insert(1);
            }
        }
    }


    fn into_next(w: &char, board_map: HashMap<&char, i32>) -> bool {
        false
    }


    false

}


#[test]
fn test_1() {
    
}