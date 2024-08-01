/// LCP 40. 心算挑战
/// 「力扣挑战赛」心算项目的挑战比赛中，要求选手从 N 张卡牌中选出 cnt 张卡牌，
/// 若这 cnt 张卡牌数字总和为偶数，则选手成绩「有效」且得分为 cnt 张卡牌数字总和。
/// 给定数组 cards 和 cnt，其中 cards[i] 表示第 i 张卡牌上的数字。
///  请帮参赛选手计算最大的有效得分。若不存在获取有效得分的卡牌方案，则返回 0。
///

pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
    let mut cards = cards.clone();
    cards.sort_by(|a, b| b.cmp(&a));

    println!("{:?}", cards);

    let mut sum = 0;
    let mut queue = vec![];
    let mut min_odd = None;
    let mut min_even = None;
    for n in &cards {
        if queue.len() < cnt as usize {
            if let Some(num) = min_odd {
                if num >= *n && n % 2 == 1 {
                    min_odd = Some(*n);
                }
            } else if n % 2 == 1 {
                min_odd = Some(*n);
            }
            if let Some(num) = min_even {
                if num >= *n && n % 2 == 0 {
                    min_even = Some(*n);
                }
            } else if n % 2 == 0 {
                min_even = Some(*n);
            }
            queue.push(n);
            sum += n;
        }
    }

    println!("{:?} ======= {:?}", min_odd, min_even);

    if sum % 2 == 0 {
        return sum;
    }
    let mut max_odd = None;
    let mut max_even = None;
    for i in cnt..(cards.len() as i32) {
        let n = cards[i as usize];
        if max_odd.is_none() {
            max_odd = Some(n);
        }
        if max_even.is_none() {
            max_even = Some(n);
        }
        if max_odd.is_some() && max_even.is_some() {
            break;
        }
    }

    println!("{:?} === {:?}", max_odd, max_even);

    let mut split_1 = 0;
    let mut split_2 = 0;
    if max_odd.is_some() && min_even.is_some() {
        split_1 = min_even.unwrap() - max_odd.unwrap();
    }
    if max_even.is_some() && min_odd.is_some() {
        split_2 = min_odd.unwrap() - max_even.unwrap();
    }

    sum - split_1.max(split_2)
}

#[test]
fn test_1() {
    assert_eq!(maxmium_score(vec![1, 2, 8, 9], 3), 18);
}

#[test]
fn test_2() {
    assert_eq!(maxmium_score(vec![3, 3, 1], 1), 0);
}
