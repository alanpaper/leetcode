
pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut numlist = vec![0;200];
    let mut result = vec![];
    let mut max_num = 0;
    nums.iter().for_each(|x| {
        numlist[*x as usize] += 1;
        max_num = std::cmp::max(max_num, numlist[*x as usize]);
    });

    for i in 0..max_num {
        let mut row = vec![];
        for j in 0..numlist.len() {
            if numlist[j] > 0 {
                row.push(j as i32);
                numlist[j] -= 1;
            }
        }
        result.push(row);
    }
    result
}

#[test]
fn test() {
    assert_eq!(find_matrix(vec![1, 3, 4, 1, 2, 3, 1]), vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]]);
}