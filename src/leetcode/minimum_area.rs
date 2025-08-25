
/// 3195. 包含所有1的最小矩形面积

pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {

    let m = grid.len();
    let n = grid[0].len();

    let mut top_min = m+1;
    let mut top_max = 0;
    let mut left_min = n+1;
    let mut left_max = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {

                left_min = left_min.min(j);
                left_max = left_max.max(j);

                top_min = top_min.min(i);
                top_max = top_max.max(i);
            }
        }
    }

    let width = left_max - left_min + 1;
    let height = top_max - top_min + 1;

    (width * height) as i32
}


#[test]
fn test() {
    assert_eq!(minimum_area(
        vec![
            vec![0,1,0],
            vec![1,0,1]
        ]
    ), 6)
}
#[test]
fn test_1() {
    assert_eq!(minimum_area(
        vec![
            vec![1],
        ]
    ), 1)
}

#[test]
fn test_2() {
    assert_eq!(minimum_area(
        vec![
            vec![0,0],vec![1,0],
        ]
    ), 1)
}


#[test]
fn test_3() {
    assert_eq!(minimum_area(
        vec![
            vec![0,0,0],
            vec![0,0,0],
            vec![0,0,1],
            vec![0,1,0]
        ]
    ), 4)
}