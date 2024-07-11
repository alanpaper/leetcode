/// 57. 插入区间
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];

    let mut merge = vec![];
    let new_first = new_interval[0];
    let new_last = new_interval[1];

    let mut is_push = false;

    for i in intervals.clone() {
        if i[1] < new_first && merge.len() == 0 {
            ans.push(i);
        } else if i[0] > new_last && merge.len() == 0 {
            if !is_push {
                is_push = true;
                ans.push(new_interval.clone());
            }
            ans.push(i);
        } else if i[1] <= new_last && merge.len() == 0 {
            merge.push(i[0].min(new_first));
            merge.push(new_last);
        } else if i[1] >= new_last && merge.len() == 0 {
            merge.push(i[0].min(new_first));
            merge.push(i[1]);
        } else {
            match merge.last() {
                Some(&x) => {
                    if x < i[0] {
                        ans.push(merge.clone());
                        ans.push(i);
                        is_push = true;
                        merge = vec![];
                    } else if x == i[0] {
                        merge.pop();
                        merge.push(i[1]);
                        ans.push(merge.clone());
                        is_push = true;
                        merge = vec![];
                    } else if x < i[1] {
                        merge.pop();
                        merge.push(i[1]);
                        ans.push(merge.clone());
                        is_push = true;
                        merge = vec![];
                    }
                }
                None => {
                    ans.push(i);
                }
            }
        }
    }

    match intervals.last().as_ref() {
        Some(&x) => {
            if x[1] < new_first {
                ans.push(new_interval.clone());
            }
        }
        None => {}
    }

    if merge.len() > 0 {
        ans.push(merge.clone())
    }

    if intervals.len() == 0 {
        ans.push(new_interval);
    }

    ans
}

#[test]
fn insert_test() {
    let words = vec![vec![1, 5], vec![6, 8]];
    assert_eq!(insert(words, vec![3, 7]), vec![vec![1, 8]]);
}
#[test]
fn insert_test_2() {
    let words = vec![vec![1, 5]];
    assert_eq!(insert(words, vec![0, 0]), vec![vec![0, 0], vec![1, 5]]);
}
#[test]
fn insert_test_1() {
    let words = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    assert_eq!(
        insert(words, vec![4, 8]),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
}
