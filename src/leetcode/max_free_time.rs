    
/// 3439. 重新安排会议得到最多空余时间 I

pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let mut emptys = vec![];
    emptys.push(start_time[0] - 0);
    for i in 1..start_time.len() {
        emptys.push(start_time[i] - end_time[i-1]);
    }
    if let Some(last) = end_time.last() {
        emptys.push(event_time - last);
    }
    let mut ans = 0;
    let mut s = 0;
    for i in 0..emptys.len() {
        s += emptys[i];
        if i < k as usize {
            continue;
        }
        ans = ans.max(s);
        s -= emptys[i-k as usize];
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(max_free_time(5,1, vec![1,3], vec![2,5]), 2);
}

#[test]
fn test_2() {
    assert_eq!(max_free_time(10,1, vec![0,2,9], vec![1,4,10]), 6);
}

#[test]
fn test_3() {
    assert_eq!(max_free_time(5,2, vec![0,1,2,3,4], vec![1,2,3,4,5]), 0);
}

#[test]
fn test_4() {
    assert_eq!(max_free_time(21,1, vec![7,10,16], vec![10,14,18]), 7);
}

#[test]
fn test_5() {
    assert_eq!(max_free_time(96,3, vec![4,11,16,53], vec![11,16,27,77]), 45);
}


// _______1112222__33___

// _____11122233333______4444________