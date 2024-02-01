/// 2396. 被列覆盖的最多行数

pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let row_len = matrix.len();
    let col_len = matrix[0].len();
    let mut totals = vec![0; col_len];
    for i in 0..col_len {
        let mut col_total = 0;
        for row in &matrix {
            col_total += row[i];
        }
        totals[i] = col_total;
    }

    0
}

#[test]
fn test_1() {
    assert_eq!(
        maximum_rows(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
            2
        ),
        3
    );
}
