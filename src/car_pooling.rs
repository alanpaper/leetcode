/// 1094. 拼车
///
/// ### 差分数组
///
/// 1. 确定最大里程数
/// 2. 在根据最大里程数生成diff数组，对应的值为当前历程在车上的人数
/// 3. 遍历原trips数组 计算diff里程中里程对应的人数
/// 4. 遍历diff判断 走完全部里程 diff中剩余的人数
/// 5. 如果剩余的人数大于乘客座位数 则表示该条件不能满足条件
///
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut to_max = 0;

    for trip in &trips {
        to_max = to_max.max(trip[2]);
    }

    let mut diff = vec![0; (to_max + 1) as usize];

    for trip in trips {
        diff[trip[1] as usize] += trip[0];
        diff[trip[2] as usize] -= trip[0];
    }

    let mut ans = true;
    let mut count = 0;
    for num in diff {
        count += num;
        if count > capacity {
            ans = false;
            break;
        }
    }
    ans
}

#[test]
fn test() {
    assert_eq!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4), false);
}
#[test]
fn test_1() {
    assert_eq!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5), true);
}
#[test]
fn test_3() {
    assert_eq!(car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3), true);
}

#[test]
fn test_2() {
    assert_eq!(
        car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11),
        true
    );
}
