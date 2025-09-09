/// 498. 对角线遍历

pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {

    let m = mat.len();
    let n = mat[0].len();

    
    let mut ans = vec![];

    let mut x = 0;
    let mut y = 0;

    ans
    
}


#[test]
fn test() {
    assert_eq!(find_diagonal_order(
        vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ]
    ), vec![1,2,4,7,5,3,6,8,9])
}




