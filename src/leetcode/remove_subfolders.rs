use std::collections::HashSet;

/// 1233. 删除子文件夹

pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {

    let mut folder = folder.clone();

    vec![]
}


#[test]
fn test_1() {
    assert_eq!(remove_subfolders(vec![
        "/a".to_string(),
        "/a/b".to_string(),
        "/c/d".to_string(),
        "/c/d/e".to_string(),
        "/c/f".to_string()
    ]), vec![
        "/a".to_string(),
        "/c/d".to_string(),
        "/c/f".to_string()
    ])
}