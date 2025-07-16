/// 551. 学生出勤记录 I
/// 给你一个字符串 s 表示一个学生的出勤记录，其中的每个字符用来标记当天的出勤情况（缺勤、迟到、到场）。记录中只含下面三种字符：
/// 'A'：Absent，缺勤
// 'L'：Late，迟到
// 'P'：Present，到场
// 如果学生能够 同时 满足下面两个条件，则可以获得出勤奖励：
// 按 总出勤 计，学生缺勤（'A'）严格 少于两天。
// 学生 不会 存在 连续 3 天或 连续 3 天以上的迟到（'L'）记录。
// 如果学生可以获得出勤奖励，返回 true ；否则，返回 false 。
pub fn check_record(s: String) -> bool {
    let mut a_total = 0;
    let mut queue = vec![];

    for char in s.chars() {
        if char == 'A' {
            queue.clear();
            a_total += 1;
            if a_total > 1 {
                return false;
            }
        }
        if char == 'L' {
            queue.push("L");
            if queue.len() >= 3 {
                return false;
            }
        } else {
            queue.clear();
        }
    }
    true
}

/// 552. 学生出勤记录 II
/// 可以用字符串表示一个学生的出勤记录，其中的每个字符用来标记当天的出勤情况（缺勤、迟到、到场）。
/// 记录中只含下面三种字符：
/// 'A'：Absent，缺勤
/// 'L'：Late，迟到
/// 'P'：Present，到场
/// 如果学生能够 同时 满足下面两个条件，则可以获得出勤奖励：
/// 按 总出勤 计，学生缺勤（'A'）严格 少于两天。
/// 学生 不会 存在 连续 3 天或 连续 3 天以上的迟到（'L'）记录。
/// 给你一个整数 n ，表示出勤记录的长度（次数）。请你返回记录长度为 n 时，
/// 可能获得出勤奖励的记录情况 数量 。答案可能很大，所以返回对 109 + 7 取余 的结果。

pub fn check_record_II(n: i32) -> i32 {

    
    // 不缺勤 n
    // 缺勤一天  

    0
}

#[test]
fn test_1() {
    assert_eq!(check_record(String::from("PPALLP")), true);
}

#[test]
fn test_2() {
    assert_eq!(check_record(String::from("PPALLL")), false);
}

/// AAPPLL

#[test]
fn test_3() {
    assert_eq!(check_record_II(2), 8);
}
#[test]
fn test_4() {
    assert_eq!(check_record_II(1), 3);
}

// "" "" ""
// A
// LL
// P

// A c 3 - 1

// L c 2 - 1
