use std::collections::HashMap;

/// 149. 直线上最多的点
///
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() < 2 {
        return 0;
    }

    let mut max = 0;

    for i in 0..(points.len() - 1) {
        let mut hash_map = HashMap::new();
        if let Some(point_x) = points.get(i) {
            for j in (i + 1)..points.len() {
                if let Some(point_y) = points.get(j) {
                    let proportion;
                    if (point_y[0] - point_x[0]) == 0 {
                        proportion = point_y[0].to_string() + "x";
                    } else if point_y[1] - point_x[1] == 0 {
                        proportion = point_y[1].to_string() + "y";
                    } else {
                        proportion = ((point_y[1] - point_x[1]) as f64
                            / (point_y[0] - point_x[0]) as f64)
                            .to_string();
                    }
                    hash_map
                        .entry(proportion)
                        .and_modify(|f| *f += 1)
                        .or_insert(2);
                }
            }
            let mut max_cur = 0;
            hash_map.iter().for_each(|f| {
                if *f.1 > max_cur {
                    max_cur = *f.1;
                }
            });

            println!("{:?}", hash_map);

            if max_cur > max {
                max = max_cur;
            }
        }
    }

    max
}

#[test]
fn test_1() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(max_points(points), 3);
}
#[test]
fn test_2() {
    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    assert_eq!(max_points(points), 4);
}

#[test]
fn test_3() {
    let points = vec![
        vec![0, 0],
        vec![4, 5],
        vec![7, 8],
        vec![8, 9],
        vec![5, 6],
        vec![3, 4],
        vec![1, 1],
    ];
    assert_eq!(max_points(points), 5);
}

#[test]
fn test_4() {
    let points = vec![vec![2, 3], vec![3, 3], vec![-5, 3]];
    assert_eq!(max_points(points), 3);
}
#[test]
fn test_5() {
    let points = vec![
        vec![7, 3],
        vec![19, 19],
        vec![-16, 3],
        vec![13, 17],
        vec![-18, 1],
        vec![-18, -17],
        vec![13, -3],
        vec![3, 7],
        vec![-11, 12],
        vec![7, 19],
        vec![19, -12],
        vec![20, -18],
        vec![-16, -15],
        vec![-10, -15],
        vec![-16, -18],
        vec![-14, -1],
        vec![18, 10],
        vec![-13, 8],
        vec![7, -5],
        vec![-4, -9],
        vec![-11, 2],
        vec![-9, -9],
        vec![-5, -16],
        vec![10, 14],
        vec![-3, 4],
        vec![1, -20],
        vec![2, 16],
        vec![0, 14],
        vec![-14, 5],
        vec![15, -11],
        vec![3, 11],
        vec![11, -10],
        vec![-1, -7],
        vec![16, 7],
        vec![1, -11],
        vec![-8, -3],
        vec![1, -6],
        vec![19, 7],
        vec![3, 6],
        vec![-1, -2],
        vec![7, -3],
        vec![-6, -8],
        vec![7, 1],
        vec![-15, 12],
        vec![-17, 9],
        vec![19, -9],
        vec![1, 0],
        vec![9, -10],
        vec![6, 20],
        vec![-12, -4],
        vec![-16, -17],
        vec![14, 3],
        vec![0, -1],
        vec![-18, 9],
        vec![-15, 15],
        vec![-3, -15],
        vec![-5, 20],
        vec![15, -14],
        vec![9, -17],
        vec![10, -14],
        vec![-7, -11],
        vec![14, 9],
        vec![1, -1],
        vec![15, 12],
        vec![-5, -1],
        vec![-17, -5],
        vec![15, -2],
        vec![-12, 11],
        vec![19, -18],
        vec![8, 7],
        vec![-5, -3],
        vec![-17, -1],
        vec![-18, 13],
        vec![15, -3],
        vec![4, 18],
        vec![-14, -15],
        vec![15, 8],
        vec![-18, -12],
        vec![-15, 19],
        vec![-9, 16],
        vec![-9, 14],
        vec![-12, -14],
        vec![-2, -20],
        vec![-3, -13],
        vec![10, -7],
        vec![-2, -10],
        vec![9, 10],
        vec![-1, 7],
        vec![-17, -6],
        vec![-15, 20],
        vec![5, -17],
        vec![6, -6],
        vec![-11, -8],
    ];
    assert_eq!(max_points(points), 6);
}
