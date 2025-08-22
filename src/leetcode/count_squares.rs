/// 1277. 统计全为 1 的正方形子矩阵
pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 1 {
                ans += 1;
            } else {
                continue;
            }
            let max_size = m - i;
            if max_size >= 1 {
                for a in 1..max_size {
                    if is_square(i, j, m, n, &matrix, a) {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans
}

fn is_square(i: usize, j: usize, m: usize, n: usize, matrix: &Vec<Vec<i32>>, size: usize) -> bool {
    if i + size > m - 1 {
        return false;
    }
    if j + size > n - 1 {
        return false;
    }
    for a in i..=(i+size) {
        for b in j..=(j+size) {
            if matrix[a][b] == 0 {
                return false;
            }
        }
    }
    true
}




#[test]
fn test_1() {
    let matrix = vec![
        vec![0,1,1,1],
        vec![1,1,1,1],
        vec![0,1,1,1]
    ];
    assert_eq!(count_squares(matrix), 15)
}

#[test]
fn test_2() {
    let matrix = vec![
        vec![1,0,1],
        vec![1,1,0],
        vec![1,1,0]
    ];
    assert_eq!(count_squares(matrix), 7)
}