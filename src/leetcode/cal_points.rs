/// 682. 棒球比赛
/// 你现在是一场采用特殊赛制棒球比赛的记录员。这场比赛由若干回合组成，过去几回合的得分可能会影响以后几回合的得分。
// 比赛开始时，记录是空白的。你会得到一个记录操作的字符串列表 ops，其中 ops[i] 是你需要记录的第 i 项操作，ops 遵循下述规则：
// 整数 x - 表示本回合新获得分数 x
// "+" - 表示本回合新获得的得分是前两次得分的总和。题目数据保证记录此操作时前面总是存在两个有效的分数。
// "D" - 表示本回合新获得的得分是前一次得分的两倍。题目数据保证记录此操作时前面总是存在一个有效的分数。
// "C" - 表示前一次得分无效，将其从记录中移除。题目数据保证记录此操作时前面总是存在一个有效的分数。
// 请你返回记录中所有得分的总和。
///
pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut queue = vec![];
    for n in operations {
        let len = queue.len();
        if *n == String::from("+") {
            let num1 = queue[len - 2] as i32;
            let num2 = queue[len - 1] as i32;
            queue.push(num1 + num2);
            continue;
        }
        if *n == String::from("D") {
            let num2 = queue[len - 1] as i32;
            queue.push(num2 * 2);
            continue;
        }
        if *n == String::from("C") {
            queue.pop();
            continue;
        }
        let num = (*n).parse::<i32>().unwrap();
        queue.push(num as i32);
    }
    queue.iter().sum()
}

#[test]
fn test1() {
    let ops = vec![
        "5".to_string(),
        "2".to_string(),
        "C".to_string(),
        "D".to_string(),
        "+".to_string(),
    ];
    assert_eq!(cal_points(ops.clone()), 30);
}

#[test]
fn test2() {
    let ops = vec![
        "5".to_string(),
        "-2".to_string(),
        "4".to_string(),
        "C".to_string(),
        "D".to_string(),
        "9".to_string(),
        "+".to_string(),
        "+".to_string(),
    ];
    assert_eq!(cal_points(ops.clone()), 30);
}
