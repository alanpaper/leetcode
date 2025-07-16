use std::collections::HashSet;

/// 1436. 旅行终点站
pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut out_tolerance = HashSet::new();
    for p in &paths {
        out_tolerance.insert(p[0].clone());
    }
    for out in &paths {
        if !out_tolerance.contains(&out[1]) {
            return out[1].clone();
        }
    }
    String::new()
}

#[test]
fn test1() {
    assert_eq!(
        dest_city(vec![
            vec![String::from("London"), String::from("New York")],
            vec![String::from("New York"), String::from("Lima")],
            vec![String::from("Lima"), String::from("Sao Paulo")]
        ]),
        String::from("Sao Paulo")
    )
}

#[test]
fn test2() {
    assert_eq!(
        dest_city(vec![
            vec![String::from("B"), String::from("C")],
            vec![String::from("D"), String::from("B")],
            vec![String::from("C"), String::from("A")]
        ]),
        String::from("A")
    )
}
