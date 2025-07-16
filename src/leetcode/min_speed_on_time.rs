/// 1870. 准时到达的列车最小时速
/// 给你一个浮点数 hour ，表示你到达办公室可用的总通勤时间。要到达办公室，你必须按给定次序乘坐 n 趟列车。
/// 另给你一个长度为 n 的整数数组 dist ，其中 dist[i] 表示第 i 趟列车的行驶距离（单位是千米）。
/// 每趟列车均只能在整点发车，所以你可能需要在两趟列车之间等待一段时间。
/// 例如，第 1 趟列车需要 1.5 小时，那你必须再等待 0.5 小时，搭乘在第 2 小时发车的第 2 趟列车。
/// 返回能满足你在时限前到达办公室所要求全部列车的 最小正整数 时速（单位：千米每小时），如果无法准时到达，则返回 -1 。
/// 生成的测试用例保证答案不超过 107 ，且 hour 的 小数点后最多存在两位数字 。
pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    let mut ans = 1;
    let mut len = dist.len();

    while ans <= 107 {
        let mut total = 0;
        for n in 0..len {
            total += dist[n] * ans;
        }
        if total <= hour as i32 {
            return ans;
        }
        if len > 1 {
            len -= 1;
        } else {
            break;
            // if (dist[0] * ans) as f64 > hour {
            //     return -1;
            // } else {
            // }
        }
        ans += 1;
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(min_speed_on_time(vec![1, 3, 2], 6.0), 1);
}
#[test]
fn test_2() {
    assert_eq!(min_speed_on_time(vec![1, 3, 2], 2.7), 3);
}
#[test]
fn test_3() {
    assert_eq!(min_speed_on_time(vec![1, 3, 2], 1.9), -1);
}
