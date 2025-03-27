pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![0, -1];
    mat.iter().enumerate().for_each(|(i, row)| {
        if res[1] < row.iter().sum() {
            res[1] = row.iter().sum();
            res[0] = i as i32;
        }
    });

    res
}
