/// 1103. 分糖果
///
pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut ans = vec![0; num_people as usize];
    let mut candies_new = candies;

    let mut n = 0;
    let mut index = 0;

    while candies_new > 0 {
        println!("{:?}", n);

        if index < num_people {
            if candies_new >= n + 1 {
                candies_new -= n + 1;
                ans[index as usize] += n + 1;
            } else {
                ans[index as usize] += candies_new;
                candies_new = 0;
            }
            index += 1;
        } else {
            index += 0;
        }
        n += 1;
    }

    ans
}

#[test]
fn test_1() {
    assert_eq!(distribute_candies(15, 4), vec![6, 2, 3, 4])
}

// #[test]
// fn test_2() {
//     assert_eq!(distribute_candies(60, 4), vec![15, 18, 15, 12])
// }
