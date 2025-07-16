/// 3200. 三角形的最大高度
/// 给你两个整数 red 和 blue，分别表示红色球和蓝色球的数量。
/// 你需要使用这些球来组成一个三角形，满足第 1 行有 1 个球，第 2 行有 2 个球，第 3 行有 3 个球，依此类推。
/// 每一行的球必须是 相同 颜色，且相邻行的颜色必须 不同。
/// 返回可以实现的三角形的 最大 高度。
///
pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    // 红球开始
    let mut red_line = f32::sqrt(red as f32).floor() as i32;

    // 红球开始
    let mut red_1 = red;
    let mut blue_1 = blue;
    let mut line_1 = 1;
    while blue_1 > 0 && red_1 > 0 {
        if line_1 % 2 == 1 {
            red_1 -= line_1;
            if red_1 <= 0 {
                break;
            }
        } else {
            blue_1 -= line_1;
            if blue_1 <= 0 {
                break;
            }
        }
        line_1 += 1;
    }
    // 绿球开始
    let mut red_2 = red;
    let mut blue_2 = blue;
    let mut line_2 = 1;
    while blue_2 > 0 && red_2 > 0 {
        if line_2 % 2 == 1 {
            red_2 -= line_2;
            if red_2 <= 0 {
                break;
            }
        } else {
            blue_2 -= line_2;
            if blue_2 <= 0 {
                break;
            }
        }
        line_2 += 1;
    }
    line_1.max(line_2)
}

#[test]
fn test_1() {
    assert_eq!(max_height_of_triangle(2, 4), 3);
}
#[test]
fn test_2() {
    assert_eq!(max_height_of_triangle(2, 1), 2);
}
#[test]
fn test_3() {
    assert_eq!(max_height_of_triangle(1, 1), 1);
}
#[test]
fn test_4() {
    assert_eq!(max_height_of_triangle(10, 1), 2);
}
