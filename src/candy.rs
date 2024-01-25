/// 135. 分发糖果

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candys = vec![1; ratings.len()];

    let mut stack = vec![];

    for i in 1..ratings.len() {
        if let Some(cur_rating) = ratings.get(i) {
            if let Some(pre_rating) = ratings.get(i - 1) {
                if cur_rating > pre_rating {
                    if stack.len() > 0 {
                        for _ in 0..stack.len() - 1 {
                            if let Some(x) = stack.pop() {
                                if let Some(_) = candys.get(x - 1) {
                                    if candys[x] + 1 > candys[x - 1] {
                                        candys[x - 1] = candys[x] + 1;
                                    }
                                }
                            }
                        }
                        stack = vec![];
                    }
                    candys[i] = candys[i - 1] + 1;
                } else if cur_rating < pre_rating {
                    if stack.len() == 0 {
                        stack.push(i - 1);
                    }
                    stack.push(i);
                }
            }
        }
    }

    if stack.len() > 0 {
        for _ in 0..stack.len() - 1 {
            if let Some(x) = stack.pop() {
                if let Some(_) = candys.get(x - 1) {
                    if candys[x] + 1 > candys[x - 1] {
                        candys[x - 1] = candys[x] + 1;
                    }
                }
            }
        }
        stack = vec![];
    }

    candys.iter().sum()
}

#[test]
fn test_1() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
}

#[test]
fn test_2() {
    assert_eq!(candy(vec![1, 2, 2]), 4);
}

#[test]
fn test_3() {
    assert_eq!(candy(vec![1, 3, 2, 2, 1]), 7);
}

#[test]
fn test_4() {
    assert_eq!(candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
}

#[test]
fn test_5() {
    assert_eq!(candy(vec![1, 3, 4, 5, 2]), 11);
}
