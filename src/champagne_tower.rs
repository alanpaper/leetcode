use std::ops::Div;
/// 799. 香槟塔
pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut ans = vec![poured as f64];
    for i in 1..=query_row {
        let mut cur_row = vec![0.0; (i + 1) as usize];
        for col in 0..i {
            let pre_row: f64 = ans[col as usize] as f64;
            if pre_row > 1.0 {
                cur_row[col as usize] += (pre_row - 1.0).div(2.0);
                cur_row[(col + 1) as usize] += (pre_row - 1.0).div(2.0);
            }
        }
        ans = cur_row;
    }
    ans[query_glass as usize].min(1.0)
}

#[test]
fn test_max_envelopes() {
    assert_eq!(champagne_tower(1, 1, 1), 0.0)
}

#[test]
fn test1_max_envelopes() {
    assert_eq!(champagne_tower(2, 1, 1), 0.5)
}
