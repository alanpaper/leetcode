/// 3169. 无需开会的工作日

pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut meetings = meetings.clone();
    meetings.sort_by(|a,b| a[0].cmp(&b[0]));

    let mut merge_meetings = vec![];
    merge_meetings.push(meetings[0].clone());
    for i in 1..meetings.len() {
        if let Some(last) = merge_meetings.last() {
            if last[1] >= meetings[i][0] {
                let start = last[0];
                let last = meetings[i][1];
                merge_meetings.pop();
                merge_meetings.push(vec![start, last]);
            } else {
                merge_meetings.push(meetings[i].clone());
            }
        }
    }

    let mut total = 0;
    for m in merge_meetings.iter() {
        total += m[1] - m[0] + 1;
    }


    days - total
    
}


#[test]
fn test_1() {
    assert_eq!(count_days(10, vec![vec![5,7],vec![1,3],vec![9,10]]), 2);
}


#[test]
fn test_2() {
    assert_eq!(count_days(10, vec![
        vec![3,49],vec![23,44],vec![21,56],vec![26,55],[23,52],[2,9],[1,48],[3,31]
    ]), 2);
}

